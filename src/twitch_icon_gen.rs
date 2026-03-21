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

const MAX_SIZE_BYTES: u64 = 25 * 1024;

#[derive(Clone)]
pub struct TwitchIconGenManager {
    jobs: Arc<RwLock<HashMap<String, JobRecord>>>,
    output_dir: PathBuf,
}

struct JobRecord {
    zip_path: PathBuf,
    #[allow(dead_code)]
    created_at: std::time::SystemTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IconGenResponse {
    /// Unique job identifier for downloading the icon pack ZIP
    pub job_id: String,
    /// Processing results for each image/size combination
    pub results: Vec<IconResult>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IconResult {
    /// Original filename
    pub filename: String,
    /// Output size in pixels (18, 36, or 72)
    pub size: u32,
    /// Whether this size was generated successfully
    pub ok: bool,
    /// Warning message if oversized or failed
    pub warning: Option<String>,
}

impl TwitchIconGenManager {
    pub fn new() -> std::io::Result<Self> {
        let output_dir = PathBuf::from("temp_twitch_icons");
        std::fs::create_dir_all(&output_dir)?;
        Ok(Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            output_dir,
        })
    }

    /// Process uploaded images through ImageMagick and package into a ZIP.
    /// `images` is a list of (original filename, raw bytes).
    /// `sizes` is the list of pixel dimensions to generate (subset of [18, 36, 72]).
    pub async fn generate_icon_pack(
        &self,
        images: Vec<(String, Vec<u8>)>,
        sizes: Vec<u32>,
    ) -> Result<(String, Vec<IconResult>), String> {
        let job_id = Uuid::new_v4().to_string();
        let job_dir = self.output_dir.join(&job_id);
        fs::create_dir_all(&job_dir)
            .await
            .map_err(|e| format!("Failed to create job directory: {}", e))?;

        let mut results: Vec<IconResult> = Vec::new();
        // Collect (zip entry name, PNG bytes) to later build the archive
        let mut zip_entries: Vec<(String, Vec<u8>)> = Vec::new();

        for (original_name, data) in &images {
            let stem = std::path::Path::new(original_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image")
                .to_string();

            let ext = std::path::Path::new(original_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png");

            let input_path = job_dir.join(format!("in_{}.{}", stem, ext));
            fs::write(&input_path, data)
                .await
                .map_err(|e| format!("Failed to write input file: {}", e))?;

            for &size in &sizes {
                let out_filename = format!("{}_{}x{}.png", stem, size, size);
                let out_path = job_dir.join(&out_filename);

                let output = Command::new("convert")
                    .arg(&input_path)
                    .arg("-resize")
                    .arg(format!("{}x{}", size, size))
                    .arg("-background")
                    .arg("none")
                    .arg("-alpha")
                    .arg("set")
                    .arg("-fuzz")
                    .arg("10%")
                    .arg("-transparent")
                    .arg("white")
                    .arg("-gravity")
                    .arg("center")
                    .arg("-extent")
                    .arg(format!("{}x{}", size, size))
                    .arg("-strip")
                    .arg("-define")
                    .arg("png:compression-level=9")
                    .arg(&out_path)
                    .output()
                    .await
                    .map_err(|e| format!("Failed to spawn convert: {}", e))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    tracing::warn!("convert failed for {}: {}", out_filename, err.trim());
                    results.push(IconResult {
                        filename: original_name.clone(),
                        size,
                        ok: false,
                        warning: Some(format!("ImageMagick error: {}", err.trim())),
                    });
                    continue;
                }

                let mut png_data = match fs::read(&out_path).await {
                    Ok(d) => d,
                    Err(e) => {
                        results.push(IconResult {
                            filename: original_name.clone(),
                            size,
                            ok: false,
                            warning: Some(format!("Failed to read output: {}", e)),
                        });
                        continue;
                    }
                };

                // If still over the limit after stripping metadata, reduce the colour
                // palette in steps until it fits (extremely unlikely for ≤72×72 PNGs).
                if png_data.len() as u64 > MAX_SIZE_BYTES {
                    for &colors in &[256u32, 128, 64, 32, 16] {
                        let reduce = Command::new("convert")
                            .arg(&out_path)
                            .arg("-colors")
                            .arg(colors.to_string())
                            .arg("-strip")
                            .arg("-define")
                            .arg("png:compression-level=9")
                            .arg(&out_path)
                            .output()
                            .await
                            .map_err(|e| format!("Failed to spawn convert for reduction: {}", e))?;
                        if reduce.status.success() {
                            if let Ok(d) = fs::read(&out_path).await {
                                png_data = d;
                                if png_data.len() as u64 <= MAX_SIZE_BYTES {
                                    break;
                                }
                            }
                        }
                    }
                }

                let _ = fs::remove_file(&out_path).await;

                let size_bytes = png_data.len() as u64;
                let warning = if size_bytes > MAX_SIZE_BYTES {
                    Some(format!(
                        "{}KB exceeds Twitch's 25KB badge limit after compression",
                        (size_bytes + 1023) / 1024
                    ))
                } else {
                    None
                };

                results.push(IconResult {
                    filename: original_name.clone(),
                    size,
                    ok: true,
                    warning,
                });

                zip_entries.push((out_filename, png_data));
            }

            let _ = fs::remove_file(&input_path).await;
        }

        let _ = fs::remove_dir(&job_dir).await;

        // Build ZIP archive in a blocking task
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
                tracing::info!("Cleaned up icon pack: {}", job_id_clone);
            }
        });

        Ok((job_id, results))
    }

    pub async fn get_zip_file(&self, job_id: &str) -> Option<PathBuf> {
        let jobs = self.jobs.read().await;
        jobs.get(job_id).map(|r| r.zip_path.clone())
    }
}
