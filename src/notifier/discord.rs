use async_trait::async_trait;
use anyhow::Result;

use crate::notifier::Notifier;
use crate::types::Notification;

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

    async fn notify(&self, notification: &Notification) -> Result<()>{
        let mut embed = serde_json::json!({
            "description": &notification.message,
            "color": 0x66ffcc
        });

        if notification.image.is_some() {
            embed["image"] = serde_json::json!({ "url": "attachment://cover.jpg" });
        }

        let payload = serde_json::json!({ "embeds": [embed] });

        let mut form = reqwest::multipart::Form::new()
            .text("payload_json", payload.to_string());

        if let Some(image_data) = &notification.image {
            let part = reqwest::multipart::Part::bytes(image_data.clone())
                .file_name("cover.jpg")
                .mime_str("image/jpeg")?;
            form = form.part("files[0]", part);
        }

        self.client.post(&self.webhook_url).multipart(form).send().await?;
        Ok(())
    }
}