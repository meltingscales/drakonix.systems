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
        Ok(DogboxLiteManager {
            uploads: Arc::new(RwLock::new(HashMap::new())),
            upload_dir,
        })
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
