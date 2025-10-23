use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::RwLock;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone)]
pub struct ConverterManager {
    conversions: Arc<RwLock<HashMap<String, ConversionRecord>>>,
    output_dir: PathBuf,
}

struct ConversionRecord {
    file_path: PathBuf,
    created_at: std::time::SystemTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConvertResponse {
    /// Unique file identifier for downloading the converted MP3
    pub file_id: String,
}

impl ConverterManager {
    pub fn new() -> std::io::Result<Self> {
        let output_dir = PathBuf::from("temp_conversions");
        std::fs::create_dir_all(&output_dir)?;

        Ok(ConverterManager {
            conversions: Arc::new(RwLock::new(HashMap::new())),
            output_dir,
        })
    }

    pub async fn convert_mp4_to_mp3(
        &self,
        input_data: Vec<u8>,
        bitrate: &str,
    ) -> Result<String, String> {
        // Validate bitrate
        let bitrate = match bitrate {
            "320" | "192" | "128" => bitrate,
            _ => "192", // default
        };

        let file_id = Uuid::new_v4().to_string();
        let input_path = self.output_dir.join(format!("{}.mp4", file_id));
        let output_path = self.output_dir.join(format!("{}.mp3", file_id));

        // Write input file
        fs::write(&input_path, input_data)
            .await
            .map_err(|e| format!("Failed to write input file: {}", e))?;

        // Run ffmpeg conversion
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_path)
            .arg("-vn") // No video
            .arg("-ar")
            .arg("44100") // Sample rate
            .arg("-ac")
            .arg("2") // Stereo
            .arg("-b:a")
            .arg(format!("{}k", bitrate)) // Bitrate
            .arg("-y") // Overwrite output
            .arg(&output_path)
            .output()
            .await
            .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;

        // Clean up input file
        let _ = fs::remove_file(&input_path).await;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            tracing::error!("FFmpeg conversion failed: {}", error);
            return Err(format!("Conversion failed: {}", error));
        }

        // Store conversion record
        {
            let mut conversions = self.conversions.write().await;
            conversions.insert(
                file_id.clone(),
                ConversionRecord {
                    file_path: output_path,
                    created_at: std::time::SystemTime::now(),
                },
            );
        }

        // Spawn cleanup task (delete after 10 minutes)
        let file_id_clone = file_id.clone();
        let conversions_clone = self.conversions.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
            if let Some(record) = conversions_clone.write().await.remove(&file_id_clone) {
                let _ = fs::remove_file(record.file_path).await;
                tracing::info!("Cleaned up conversion file: {}", file_id_clone);
            }
        });

        Ok(file_id)
    }

    pub async fn get_conversion_file(&self, file_id: &str) -> Option<PathBuf> {
        let conversions = self.conversions.read().await;
        conversions.get(file_id).map(|record| record.file_path.clone())
    }

    pub async fn cleanup_old_files(&self) {
        let mut conversions = self.conversions.write().await;
        let now = std::time::SystemTime::now();
        let mut to_remove = Vec::new();

        for (id, record) in conversions.iter() {
            if let Ok(duration) = now.duration_since(record.created_at) {
                if duration.as_secs() > 600 {
                    // 10 minutes
                    to_remove.push(id.clone());
                }
            }
        }

        for id in to_remove {
            if let Some(record) = conversions.remove(&id) {
                let _ = fs::remove_file(record.file_path).await;
                tracing::info!("Cleaned up old conversion: {}", id);
            }
        }
    }
}
