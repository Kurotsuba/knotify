use async_trait::async_trait;
use anyhow::Result;

use crate::platform::Platform;
use crate::config::ChannelConfig;
use crate::types::StreamInfo;

pub struct BilibiliPlatform {
    client: reqwest::Client,
}

impl BilibiliPlatform {
    pub fn new() -> Self {
        BilibiliPlatform { client: reqwest::Client::new() }
    }
}

#[async_trait]
impl Platform for BilibiliPlatform {
    fn name(&self) -> &str {
        "bilibili"
    }

    async fn check_live(&self, channel: &ChannelConfig) -> Result<Option<StreamInfo>> {
        let resp = self.client
            .get("https://api.live.bilibili.com/room/v1/Room/get_info")
            .query(&[("room_id", &channel.channel_id)])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let live_status = resp["data"]["live_status"].as_i64().unwrap_or(0);
        if live_status != 1 {
            return Ok(None);
        }

        let title = resp["data"]["title"].as_str().unwrap_or_default();
        let cover = resp["data"]["user_cover"].as_str()
            .or_else(|| resp["data"]["keyframe"].as_str())
            .map(|s| s.to_string());

        Ok(Some(StreamInfo {
            platform: "bilibili".to_string(),
            channel_id: channel.channel_id.clone(),
            channel_name: channel.name.clone(),
            title: title.to_string(),
            url: format!("https://live.bilibili.com/{}", channel.channel_id),
            cover,
        }))
    }
}