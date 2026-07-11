use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

const TTL_SECS: u64 = 30 * 24 * 3600; // 30 days

#[derive(Deserialize)]
pub struct CreatePasteRequest {
    pub content: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "text".to_string()
}

#[derive(Serialize)]
pub struct CreatePasteResponse {
    pub id: String,
}

#[derive(Clone)]
pub struct PasteRecord {
    pub content: String,
    pub language: String,
}

pub struct PasteInfo {
    pub content: String,
    pub language: String,
}

#[derive(Clone)]
pub struct DoggyPastebinManager {
    pastes: Arc<RwLock<HashMap<Uuid, PasteRecord>>>,
    paste_dir: PathBuf,
    syntax_set: Arc<SyntaxSet>,
    theme_set: Arc<ThemeSet>,
}

/// A paste is stored on disk as `<uuid>.paste`: first line is the language
/// token, remaining lines are the raw paste content.
fn encode(language: &str, content: &str) -> Vec<u8> {
    format!("{}\n{}", language, content).into_bytes()
}

fn decode(raw: &str) -> (String, String) {
    match raw.split_once('\n') {
        Some((language, content)) => (language.to_string(), content.to_string()),
        None => ("text".to_string(), raw.to_string()),
    }
}

impl DoggyPastebinManager {
    pub fn new() -> std::io::Result<Self> {
        let paste_dir = PathBuf::from("temp_doggypastebin");
        std::fs::create_dir_all(&paste_dir)?;

        // On startup, recover pastes left over from a previous run.
        // Build the map synchronously before wrapping in Arc<RwLock<>> to
        // avoid calling blocking_write() inside the tokio runtime.
        let mut initial_map: HashMap<Uuid, PasteRecord> = HashMap::new();
        let mut to_spawn: Vec<(Uuid, PathBuf, u64)> = Vec::new(); // (id, path, remaining_secs)

        let now = std::time::SystemTime::now();
        for entry in std::fs::read_dir(&paste_dir)?.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|n| n.to_str()) else {
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
                tracing::info!("doggypastebin: removed expired orphan {}", id);
            } else {
                let raw = std::fs::read_to_string(&path).unwrap_or_default();
                let (language, content) = decode(&raw);
                let remaining = TTL_SECS - age_secs;
                initial_map.insert(id, PasteRecord { content, language });
                to_spawn.push((id, path, remaining));
                tracing::info!("doggypastebin: recovered orphan {} ({} s remaining)", id, remaining);
            }
        }

        let pastes: Arc<RwLock<HashMap<Uuid, PasteRecord>>> = Arc::new(RwLock::new(initial_map));

        // Now that we're inside the runtime we can safely spawn.
        for (id, path, remaining) in to_spawn {
            let pastes_clone = pastes.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(remaining)).await;
                if pastes_clone.write().await.remove(&id).is_some() {
                    let _ = fs::remove_file(&path).await;
                } else {
                    let _ = fs::remove_file(&path).await;
                }
                tracing::info!("doggypastebin: cleaned up recovered paste {}", id);
            });
        }

        Ok(DoggyPastebinManager {
            pastes,
            paste_dir,
            syntax_set: Arc::new(SyntaxSet::load_defaults_newlines()),
            theme_set: Arc::new(ThemeSet::load_defaults()),
        })
    }

    pub async fn create_paste(&self, content: String, language: String) -> Result<Uuid, String> {
        let id = Uuid::new_v4();
        let file_path = self.paste_dir.join(format!("{}.paste", id));

        fs::write(&file_path, encode(&language, &content))
            .await
            .map_err(|e| format!("Failed to write paste: {}", e))?;

        {
            let mut pastes = self.pastes.write().await;
            pastes.insert(
                id,
                PasteRecord {
                    content,
                    language,
                },
            );
        }

        // Schedule deletion after TTL
        let id_clone = id;
        let pastes_clone = self.pastes.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(TTL_SECS)).await;
            if pastes_clone.write().await.remove(&id_clone).is_some() {
                let _ = fs::remove_file(
                    PathBuf::from("temp_doggypastebin").join(format!("{}.paste", id_clone)),
                )
                .await;
                tracing::info!("doggypastebin: cleaned up {}", id_clone);
            }
        });

        Ok(id)
    }

    pub async fn get_paste(&self, id: &Uuid) -> Option<PasteInfo> {
        let pastes = self.pastes.read().await;
        pastes.get(id).map(|r| PasteInfo {
            content: r.content.clone(),
            language: r.language.clone(),
        })
    }

    /// Render a paste's content as syntax-highlighted HTML.
    pub fn highlight(&self, content: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        highlighted_html_for_string(content, &self.syntax_set, syntax, theme).unwrap_or_else(|_| {
            format!(
                "<pre><code>{}</code></pre>",
                html_escape::encode_text(content)
            )
        })
    }
}
