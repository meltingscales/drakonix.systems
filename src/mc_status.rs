use base64::Engine;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

const CACHE_TTL: Duration = Duration::from_secs(60);
const PING_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Serialize)]
pub struct ServerStatus {
    pub name: String,
    pub address: String,
    pub online: bool,
    pub motd: Option<String>,
    pub players_online: Option<usize>,
    pub players_max: Option<usize>,
    pub version: Option<String>,
    pub favicon: Option<String>,
    pub error: Option<String>,
}

/// A server to monitor: display name + Server List Ping address.
#[derive(Debug, Clone, Copy)]
pub struct ServerSpec {
    pub name: &'static str,
    pub host: &'static str,
    pub port: u16,
}

pub const SERVERS: &[ServerSpec] = &[
    ServerSpec {
        name: "Liminal Industries",
        host: "meltyliminalindustries.playit.plus",
        port: 44940,
    },
    ServerSpec {
        name: "Drakonix Tech Pack",
        host: "drakonixtechpack.playit.plus",
        port: 23387,
    },
];

/// Caches Server List Ping results per host:port for CACHE_TTL, so a page
/// load never triggers more than one live ping per server per minute.
#[derive(Clone)]
pub struct McStatusCache {
    inner: Arc<RwLock<HashMap<String, (Instant, ServerStatus)>>>,
}

impl McStatusCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_all(&self) -> Vec<ServerStatus> {
        let mut out = Vec::with_capacity(SERVERS.len());
        for spec in SERVERS {
            out.push(self.get(spec).await);
        }
        out
    }

    async fn get(&self, spec: &ServerSpec) -> ServerStatus {
        let key = format!("{}:{}", spec.host, spec.port);

        if let Some((fetched_at, status)) = self.inner.read().await.get(&key) {
            if fetched_at.elapsed() < CACHE_TTL {
                return status.clone();
            }
        }

        let status = ping(spec).await;
        self.inner
            .write()
            .await
            .insert(key, (Instant::now(), status.clone()));
        status
    }
}

impl Default for McStatusCache {
    fn default() -> Self {
        Self::new()
    }
}

async fn ping(spec: &ServerSpec) -> ServerStatus {
    let address = format!("{}:{}", spec.host, spec.port);

    match tokio::time::timeout(PING_TIMEOUT, ping_inner(spec.host, spec.port)).await {
        Ok(Ok(response)) => ServerStatus {
            name: spec.name.to_string(),
            address,
            online: true,
            motd: response.description.as_ref().map(motd_to_text),
            players_online: Some(response.online_players),
            players_max: Some(response.max_players),
            version: Some(response.version),
            favicon: response
                .favicon
                .map(|bytes| format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))),
            error: None,
        },
        Ok(Err(e)) => offline(spec, address, e.to_string()),
        Err(_) => offline(spec, address, "timed out".to_string()),
    }
}

fn offline(spec: &ServerSpec, address: String, error: String) -> ServerStatus {
    ServerStatus {
        name: spec.name.to_string(),
        address,
        online: false,
        motd: None,
        players_online: None,
        players_max: None,
        version: None,
        favicon: None,
        error: Some(error),
    }
}

async fn ping_inner(host: &str, port: u16) -> Result<craftping::Response, craftping::Error> {
    let mut stream = TcpStream::connect((host, port))
        .await
        .map_err(craftping::Error::Io)?;
    craftping::tokio::ping(&mut stream, host, port, craftping::PROTOCOL_VERSION_NOT_SET).await
}

/// Server MOTDs are Minecraft chat components (a plain string, or an object
/// with "text" plus nested "extra" components) — flatten to plain text.
fn motd_to_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(map) => {
            let mut out = map
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if let Some(extra) = map.get("extra").and_then(|v| v.as_array()) {
                for part in extra {
                    out.push_str(&motd_to_text(part));
                }
            }
            out
        }
        _ => String::new(),
    }
}
