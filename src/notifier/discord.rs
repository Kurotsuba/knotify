use async_trait::async_trait;
use anyhow::Result;

use crate::notifier::Notifier;
use crate::types::StreamInfo;

pub struct DiscordNotifier {
    client: reqwest::Client,
    webhook_url: String,
}

impl DiscordNotifier {
    pub fn new(webhook_url: String) -> Self {
        DiscordNotifier {
            client: reqwest::Client::new(),
            webhook_url,
        }
    }
}

#[async_trait]
impl Notifier for DiscordNotifier {
    fn name(&self) ->  &str {
        "discord"
    }

    async fn notify(&self, info: &StreamInfo) -> Result<()>{
        let mut embed = serde_json::json!({
            "description": format!(
                "【直播提醒】\n{}开播啦\n{}\n直播间地址：{}",
                info.channel_name, info.title, info.url
            ),
            "color": 0x66ffcc
        });

        if let Some(cover) = &info.cover {
            embed["image"] = serde_json::json!({ "url": cover });
        }

        let body = serde_json::json!({ "embeds": [embed] });

        self.client.post(&self.webhook_url).json(&body).send().await?;
        Ok(())
    }
}