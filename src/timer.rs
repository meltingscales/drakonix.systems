use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Clone)]
pub struct TimerManager {
    timers: Arc<RwLock<HashMap<String, TimerHandle>>>,
}

struct TimerHandle {
    cancel_tx: tokio::sync::mpsc::Sender<()>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTimerRequest {
    pub duration_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct StartTimerResponse {
    pub timer_id: String,
    pub duration_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct TimerStatusResponse {
    pub timer_id: String,
    pub is_active: bool,
}

impl TimerManager {
    pub fn new() -> Self {
        TimerManager {
            timers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_timer(&self, duration_seconds: u64) -> String {
        let timer_id = Uuid::new_v4().to_string();
        let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel::<()>(1);

        let handle = TimerHandle { cancel_tx };

        // Store the timer handle
        {
            let mut timers = self.timers.write().await;
            timers.insert(timer_id.clone(), handle);
        }

        // Spawn background task to wait for timer completion
        let timer_id_clone = timer_id.clone();
        let timers_clone = self.timers.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = sleep(Duration::from_secs(duration_seconds)) => {
                    tracing::info!("Timer {} completed", timer_id_clone);
                    // Remove from active timers
                    let mut timers = timers_clone.write().await;
                    timers.remove(&timer_id_clone);
                }
                _ = cancel_rx.recv() => {
                    tracing::info!("Timer {} cancelled", timer_id_clone);
                    // Remove from active timers
                    let mut timers = timers_clone.write().await;
                    timers.remove(&timer_id_clone);
                }
            }
        });

        timer_id
    }

    pub async fn cancel_timer(&self, timer_id: &str) -> bool {
        let mut timers = self.timers.write().await;
        if let Some(handle) = timers.remove(timer_id) {
            // Send cancel signal (ignore error if receiver already dropped)
            let _ = handle.cancel_tx.send(()).await;
            true
        } else {
            false
        }
    }

    pub async fn is_timer_active(&self, timer_id: &str) -> bool {
        let timers = self.timers.read().await;
        timers.contains_key(timer_id)
    }

    pub async fn active_timer_count(&self) -> usize {
        let timers = self.timers.read().await;
        timers.len()
    }
}
