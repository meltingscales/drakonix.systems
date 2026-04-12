use serde::Serialize;
use std::collections::HashMap;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::RwLock;
use utoipa::ToSchema;
use uuid::Uuid;

/// 1 MB limit for PNG emote outputs
const MAX_PNG_BYTES: u64 = 1024 * 1024;
/// 512 KB limit for each GIF emote output (manual-upload mode requirement)
const MAX_GIF_BYTES: u64 = 512 * 1024;
/// Twitch maximum animated emote frame count
const MAX_GIF_FRAMES: u64 = 60;

#[derive(Clone)]
pub struct TwitchEmoteGenManager {
    jobs: Arc<RwLock<HashMap<String, JobRecord>>>,
    output_dir: PathBuf,
}

struct JobRecord {
    zip_path: PathBuf,
    #[allow(dead_code)]
    created_at: std::time::SystemTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmoteGenResponse {
    /// Unique job identifier for downloading the emote pack ZIP
    pub job_id: String,
    /// Processing results for each image/size combination
    pub results: Vec<EmoteResult>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmoteResult {
    /// Original filename
    pub filename: String,
    /// Output size in pixels (28, 56, or 112)
    pub size: u32,
    /// Whether this size was generated successfully
    pub ok: bool,
    /// Warning or error message if applicable
    pub warning: Option<String>,
}

impl TwitchEmoteGenManager {
    pub fn new() -> std::io::Result<Self> {
        let output_dir = PathBuf::from("temp_twitch_emotes");
        std::fs::create_dir_all(&output_dir)?;
        Ok(Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            output_dir,
        })
    }

    /// Returns the number of frames in a GIF using ImageMagick `identify`.
    async fn gif_frame_count(path: &PathBuf) -> Result<u64, String> {
        let output = Command::new("identify")
            .arg(path)
            .output()
            .await
            .map_err(|e| format!("Failed to run identify: {}", e))?;

        // Each line of `identify` output corresponds to one frame.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.lines().count() as u64;
        Ok(count.max(1))
    }

    /// Process uploaded emote images and package them into a ZIP.
    /// `images` is a list of (original filename, raw bytes).
    /// `sizes` is the subset of [28, 56, 112] to generate.
    pub async fn generate_emote_pack(
        &self,
        images: Vec<(String, Vec<u8>)>,
        sizes: Vec<u32>,
    ) -> Result<(String, Vec<EmoteResult>), String> {
        let job_id = Uuid::new_v4().to_string();
        let job_dir = self.output_dir.join(&job_id);
        fs::create_dir_all(&job_dir)
            .await
            .map_err(|e| format!("Failed to create job directory: {}", e))?;

        let mut results: Vec<EmoteResult> = Vec::new();
        let mut zip_entries: Vec<(String, Vec<u8>)> = Vec::new();

        for (original_name, data) in &images {
            let stem = std::path::Path::new(original_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("emote")
                .to_string();

            let ext = std::path::Path::new(original_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png")
                .to_lowercase();

            let is_gif = ext == "gif";
            let is_apng = !is_gif && ext == "png" && Self::detect_apng(data);

            let input_path = job_dir.join(format!("in_{}.{}", stem, ext));
            fs::write(&input_path, data)
                .await
                .map_err(|e| format!("Failed to write input file: {}", e))?;

            // Validate animated frame count before processing
            if is_gif || is_apng {
                let label = if is_gif { "GIF" } else { "APNG" };
                match Self::gif_frame_count(&input_path).await {
                    Ok(frames) if frames > MAX_GIF_FRAMES => {
                        for &size in &sizes {
                            results.push(EmoteResult {
                                filename: original_name.clone(),
                                size,
                                ok: false,
                                warning: Some(format!(
                                    "{} has {} frames; Twitch allows a maximum of {} frames",
                                    label, frames, MAX_GIF_FRAMES
                                )),
                            });
                        }
                        let _ = fs::remove_file(&input_path).await;
                        continue;
                    }
                    Err(e) => {
                        tracing::warn!("Could not count {} frames for {}: {}", label, original_name, e);
                    }
                    Ok(_) => {}
                }
            }

            for &size in &sizes {
                let out_ext = if is_gif { "gif" } else { "png" };
                let out_filename = format!("{}_{}x{}.{}", stem, size, size, out_ext);
                let out_path = job_dir.join(&out_filename);

                let success = if is_gif {
                    Self::resize_gif(&input_path, &out_path, size).await
                } else if is_apng {
                    Self::resize_apng(&input_path, &out_path, size).await
                } else {
                    Self::resize_png(&input_path, &out_path, size).await
                };

                match success {
                    Err(e) => {
                        results.push(EmoteResult {
                            filename: original_name.clone(),
                            size,
                            ok: false,
                            warning: Some(e),
                        });
                        continue;
                    }
                    Ok(_) => {}
                }

                let out_data = match fs::read(&out_path).await {
                    Ok(d) => d,
                    Err(e) => {
                        results.push(EmoteResult {
                            filename: original_name.clone(),
                            size,
                            ok: false,
                            warning: Some(format!("Failed to read output: {}", e)),
                        });
                        continue;
                    }
                };
                let _ = fs::remove_file(&out_path).await;

                let size_bytes = out_data.len() as u64;
                let is_animated = is_gif || is_apng;
                let limit = if is_animated { MAX_GIF_BYTES } else { MAX_PNG_BYTES };
                let warning = if size_bytes > limit {
                    Some(format!(
                        "{}KB exceeds Twitch's {}KB {} emote size limit",
                        (size_bytes + 1023) / 1024,
                        limit / 1024,
                        if is_animated { "animated" } else { "static" }
                    ))
                } else {
                    None
                };

                results.push(EmoteResult {
                    filename: original_name.clone(),
                    size,
                    ok: true,
                    warning,
                });

                zip_entries.push((out_filename, out_data));
            }

            let _ = fs::remove_file(&input_path).await;
        }

        let _ = fs::remove_dir(&job_dir).await;

        // Build ZIP archive
        let zip_path = self.output_dir.join(format!("{}.zip", job_id));
        let zip_path_clone = zip_path.clone();
        tokio::task::spawn_blocking(move || -> Result<(), String> {
            use zip::write::SimpleFileOptions;
            let file = std::fs::File::create(&zip_path_clone)
                .map_err(|e| format!("Failed to create ZIP: {}", e))?;
            let mut writer = zip::ZipWriter::new(file);
            let options = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            for (name, data) in &zip_entries {
                writer
                    .start_file(name, options)
                    .map_err(|e| format!("ZIP start_file error: {}", e))?;
                writer
                    .write_all(data)
                    .map_err(|e| format!("ZIP write error: {}", e))?;
            }
            writer
                .finish()
                .map_err(|e| format!("ZIP finish error: {}", e))?;
            Ok(())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| e)?;

        // Register the job
        {
            let mut jobs = self.jobs.write().await;
            jobs.insert(
                job_id.clone(),
                JobRecord {
                    zip_path,
                    created_at: std::time::SystemTime::now(),
                },
            );
        }

        // Schedule cleanup after 10 minutes
        let job_id_clone = job_id.clone();
        let jobs_clone = self.jobs.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
            if let Some(record) = jobs_clone.write().await.remove(&job_id_clone) {
                let _ = fs::remove_file(record.zip_path).await;
                tracing::info!("Cleaned up emote pack: {}", job_id_clone);
            }
        });

        Ok((job_id, results))
    }

    async fn resize_png(input: &PathBuf, output: &PathBuf, size: u32) -> Result<(), String> {
        let out = Command::new("convert")
            .arg(input)
            .arg("-resize")
            .arg(format!("{}x{}", size, size))
            .arg("-background")
            .arg("none")
            .arg("-alpha")
            .arg("set")
            .arg("-gravity")
            .arg("center")
            .arg("-extent")
            .arg(format!("{}x{}", size, size))
            .arg("-strip")
            .arg("-define")
            .arg("png:compression-level=9")
            .arg(output)
            .output()
            .await
            .map_err(|e| format!("Failed to spawn convert: {}", e))?;

        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("ImageMagick error: {}", err.trim()));
        }
        Ok(())
    }

    /// Returns true if the raw bytes contain an APNG `acTL` chunk.
    fn detect_apng(data: &[u8]) -> bool {
        data.windows(4).any(|w| w == b"acTL")
    }

    async fn resize_apng(input: &PathBuf, output: &PathBuf, size: u32) -> Result<(), String> {
        // Prefix output with "APNG:" so ImageMagick writes animated PNG, not static PNG.
        let output_arg = format!("APNG:{}", output.display());
        let out = Command::new("convert")
            .arg(input)
            .arg("-coalesce")
            .arg("-resize")
            .arg(format!("{}x{}", size, size))
            .arg("-background")
            .arg("none")
            .arg("-gravity")
            .arg("center")
            .arg("-extent")
            .arg(format!("{}x{}", size, size))
            .arg("-layers")
            .arg("optimize")
            .arg(&output_arg)
            .output()
            .await
            .map_err(|e| format!("Failed to spawn convert: {}", e))?;

        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("ImageMagick error: {}", err.trim()));
        }
        Ok(())
    }

    async fn resize_gif(input: &PathBuf, output: &PathBuf, size: u32) -> Result<(), String> {
        // Coalesce (fully expand frames) → resize → re-optimize layers
        let out = Command::new("convert")
            .arg(input)
            .arg("-coalesce")
            .arg("-resize")
            .arg(format!("{}x{}", size, size))
            .arg("-background")
            .arg("none")
            .arg("-gravity")
            .arg("center")
            .arg("-extent")
            .arg(format!("{}x{}", size, size))
            .arg("-layers")
            .arg("optimize")
            .arg(output)
            .output()
            .await
            .map_err(|e| format!("Failed to spawn convert: {}", e))?;

        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("ImageMagick error: {}", err.trim()));
        }
        Ok(())
    }

    pub async fn get_zip_file(&self, job_id: &str) -> Option<PathBuf> {
        let jobs = self.jobs.read().await;
        jobs.get(job_id).map(|r| r.zip_path.clone())
    }
}
