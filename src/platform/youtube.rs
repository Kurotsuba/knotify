use async_trait::async_trait;
use anyhow::Result;

use crate::platform::Platform;
use crate::config::ChannelConfig;
use crate::types::StreamInfo;

pub struct YouTubePlatform {
    client: reqwest::Client,
    api_key: String,
}

impl YouTubePlatform {
    pub fn new(api_key: String) -> Self {
        YouTubePlatform { client: reqwest::Client::new(), api_key }
    }
}

#[async_trait]
impl Platform for YouTubePlatform {
    fn name(&self) ->  &str {
        "youtube"
    }

    async fn check_live(&self, channel: &ChannelConfig) -> Result<Option<StreamInfo>> {
        let resp = self.client
            .get("https://www.googleapis.com/youtube/v3/search")
            .query(&[
                ("part", "snippet"),
                ("channelId", &channel.channel_id),
                ("eventType", "live"),
                ("type", "video"),
                ("key", &self.api_key),
            ])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let items = resp["items"].as_array();
        match items.and_then(|arr| arr.first()) {
            Some(item) => {
                let channel_name = item["snippet"]["channelTitle"].as_str().unwrap_or_default();
                let title = item["snippet"]["title"].as_str().unwrap_or_default();
                let video_id = item["id"]["videoId"].as_str().unwrap_or_default();

                Ok(Some(StreamInfo { 
                    platform: "youtube".to_string(),
                    channel_id: channel.channel_id.clone(),
                    channel_name: channel_name.to_string(),
                    title: title.to_string(),
                    url: format!("https://www.youtube.com/watch?v={video_id}")
                }))
            }
            None => Ok(None),
        }
    
    }
}