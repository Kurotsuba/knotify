use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u16,
    
    #[serde(default = "default_state_file")]
    pub state_file: String,
    
    pub youtube_api_key: String,
    pub channels: Vec<ChannelConfig>,
    pub notifiers: Vec<NotifierConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelConfig {
    pub platform: String,
    pub channel_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NotifierConfig {
    #[serde(rename = "type")]
    pub notifier_type: String,
    pub endpoint: Option<String>,
}

pub fn load_config(path: &str) -> Result<AppConfig> {
    let contents = std::fs::read_to_string(path)?;
    let config: AppConfig = serde_yml::from_str(&contents)?;
    Ok(config)
}

fn default_poll_interval() -> u16 { 300 }
fn default_state_file() -> String { "./knotify_state.json".to_string() }