use anyhow::Result;

use crate::platform::Platform;
use crate::config::ChannelConfig;
use crate::types::StreamInfo;

pub struct BilibiliPlatform;

impl BilibiliPlatform {
    pub fn new() -> Self {
        BilibiliPlatform
    }
}

impl Platform for BilibiliPlatform {
    fn name(&self) -> &str {
        "bilibili"
    }

    fn check_live(&self, channel: &ChannelConfig, agent: &ureq::Agent) -> Result<Option<StreamInfo>> {
        let resp: serde_json::Value = agent.get("https://api.live.bilibili.com/room/v1/Room/get_info")
            .query("room_id", &channel.channel_id)
            .call()?
            .into_json()?;

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