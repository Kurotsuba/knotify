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
        let body = serde_json::json!({
            "embeds": [{
                "title": format!("{} is live!", info.channel_name),
                "description": &info.title,
                "url": &info.url,
                "color": 0x66ffcc
            }]
        });

        self.client.post(&self.webhook_url).json(&body).send().await?;
        Ok(())
    }
}