use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

const TTL_SECS: u64 = 3600; // 1 hour

#[derive(Clone)]
pub struct DogboxLiteManager {
    uploads: Arc<RwLock<HashMap<Uuid, UploadRecord>>>,
    upload_dir: PathBuf,
}

struct UploadRecord {
    file_path: PathBuf,
    original_filename: String,
    content_type: String,
}

pub struct UploadInfo {
    pub original_filename: String,
    pub content_type: String,
    pub file_path: PathBuf,
}

impl DogboxLiteManager {
    pub fn new() -> std::io::Result<Self> {
        let upload_dir = PathBuf::from("temp_dogbox_lite");
        std::fs::create_dir_all(&upload_dir)?;

        // On startup, recover files left over from a previous run.
        // Build the map synchronously before wrapping in Arc<RwLock<>> to
        // avoid calling blocking_write() inside the tokio runtime.
        let mut initial_map: HashMap<Uuid, UploadRecord> = HashMap::new();
        let mut to_spawn: Vec<(Uuid, PathBuf, u64)> = Vec::new(); // (id, path, remaining_secs)

        let now = std::time::SystemTime::now();
        for entry in std::fs::read_dir(&upload_dir)?.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(stem) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Ok(id) = stem.parse::<Uuid>() else {
                continue;
            };

            let age_secs = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|mtime| now.duration_since(mtime).ok())
                .map(|d| d.as_secs())
                .unwrap_or(TTL_SECS);

            if age_secs >= TTL_SECS {
                let _ = std::fs::remove_file(&path);
                tracing::info!("dogbox-lite: removed expired orphan {}", id);
            } else {
                let remaining = TTL_SECS - age_secs;
                initial_map.insert(
                    id,
                    UploadRecord {
                        file_path: path.clone(),
                        original_filename: stem.to_string(),
                        content_type: "application/octet-stream".to_string(),
                    },
                );
                to_spawn.push((id, path, remaining));
                tracing::info!("dogbox-lite: recovered orphan {} ({} s remaining)", id, remaining);
            }
        }

        let uploads: Arc<RwLock<HashMap<Uuid, UploadRecord>>> =
            Arc::new(RwLock::new(initial_map));

        // Now that we're inside the runtime we can safely spawn.
        for (id, path, remaining) in to_spawn {
            let uploads_clone = uploads.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(remaining)).await;
                if let Some(record) = uploads_clone.write().await.remove(&id) {
                    let _ = fs::remove_file(record.file_path).await;
                } else {
                    let _ = fs::remove_file(&path).await;
                }
                tracing::info!("dogbox-lite: cleaned up recovered file {}", id);
            });
        }

        Ok(DogboxLiteManager { uploads, upload_dir })
    }

    pub async fn store_file(
        &self,
        data: Vec<u8>,
        original_filename: String,
        content_type: String,
    ) -> Result<Uuid, String> {
        let id = Uuid::new_v4();
        let file_path = self.upload_dir.join(id.to_string());

        fs::write(&file_path, &data)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        {
            let mut uploads = self.uploads.write().await;
            uploads.insert(
                id,
                UploadRecord {
                    file_path: file_path.clone(),
                    original_filename,
                    content_type,
                },
            );
        }

        // Schedule deletion after 1 hour
        let id_clone = id;
        let uploads_clone = self.uploads.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(TTL_SECS)).await;
            if let Some(record) = uploads_clone.write().await.remove(&id_clone) {
                let _ = fs::remove_file(record.file_path).await;
                tracing::info!("dogbox-lite: cleaned up {}", id_clone);
            }
        });

        Ok(id)
    }

    pub async fn get_file(&self, id: &Uuid) -> Option<UploadInfo> {
        let uploads = self.uploads.read().await;
        uploads.get(id).map(|r| UploadInfo {
            original_filename: r.original_filename.clone(),
            content_type: r.content_type.clone(),
            file_path: r.file_path.clone(),
        })
    }
}
