use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StreamKey {
    pub platform: String,
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub platform: String,
    pub channel_id: String,
    pub channel_name: String,
    pub title: String,
    pub url: String,
    pub cover: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub image: Option<Vec<u8>>,
}