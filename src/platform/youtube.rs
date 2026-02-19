use anyhow::Result;

use crate::platform::Platform;
use crate::config::ChannelConfig;
use crate::types::StreamInfo;

pub struct YouTubePlatform; 

impl YouTubePlatform {
    pub fn new() -> Self {
        YouTubePlatform {}
    }
}

impl Platform for YouTubePlatform {
    fn name(&self) -> &str {
        "youtube"
    }

    fn check_live(&self, channel: &ChannelConfig, agent: &ureq::Agent) -> Result<Option<StreamInfo>> {
        let url = if channel.channel_id.starts_with('@') {
            format!("https://www.youtube.com/{}/live", channel.channel_id)
        } else {
            format!("https://www.youtube.com/channel/{}/live", channel.channel_id)
        };

        let body = agent.get(&url)
            .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .call()?
            .into_string()?;

        let json_str = match extract_json_object(&body, "var ytInitialPlayerResponse = ") {
            Some(s) => s,
            None => return Ok(None),
        };

        let data: serde_json::Value = serde_json::from_str(&json_str)?;
        let details = &data["videoDetails"];

        let is_live = details["isLive"].as_bool().unwrap_or(false);
        if !is_live {
            return Ok(None);
        }

        let video_id = details["videoId"].as_str().unwrap_or_default();
        let title = details["title"].as_str().unwrap_or_default();
        let author = details["author"].as_str().unwrap_or(&channel.name);

        let cover = details["thumbnail"]["thumbnails"]
            .as_array()
            .and_then(|arr| arr.last())
            .and_then(|t| t["url"].as_str())
            .map(|s| s.to_string());

        Ok(Some(StreamInfo {
            platform: "youtube".to_string(),
            channel_id: channel.channel_id.clone(),
            channel_name: author.to_string(),
            title: title.to_string(),
            url: format!("https://www.youtube.com/watch?v={}", video_id),
            cover,
        }))
    }
}

fn extract_json_object(text: &str, prefix: &str) -> Option<String> {
    let start = text.find(prefix)? + prefix.len();
    let bytes = text.as_bytes();
    let mut depth = 0;
    let mut end = start;
    for i in start..bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth == 0 && end > start {
        Some(text[start..end].to_string())
    } else {
        None
    }
}
