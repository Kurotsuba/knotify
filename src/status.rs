use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LiveStatus {
    pub live: HashMap<String, bool>,
}

impl LiveStatus {
    pub fn load_status(path: &str) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save_status(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn should_notify(&mut self, platform: &str, channel_id: &str, is_live: bool) -> bool {
        if is_live {
            if self.is_known_live(platform, channel_id) {
                false
            } else {
                self.mark_live(platform, channel_id);
                true
            }
        } else {
            self.mark_offline(platform, channel_id);
            false
        }
    }

    fn is_known_live(&self, platform: &str, channel_id: &str) -> bool {
        self.live.contains_key(&format!("{platform}:{channel_id}"))
    }

    fn mark_live(&mut self, platform: &str, channel_id: &str) {
        self.live.insert(format!("{platform}:{channel_id}"), true);
    }

    fn mark_offline(&mut self, platform: &str, channel_id: &str) {
        self.live.remove(&format!("{platform}:{channel_id}"));
    }
    
}