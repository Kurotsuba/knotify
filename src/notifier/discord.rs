use anyhow::Result;

use crate::notifier::Notifier;
use crate::types::Notification;

pub struct DiscordNotifier {
        webhook_url: String,
}

impl DiscordNotifier {
    pub fn new(webhook_url: String) -> Self {
        DiscordNotifier {
            webhook_url,
        }
    }
}

impl Notifier for DiscordNotifier {
    fn name(&self) ->  &str {
        "discord"
    }

    fn notify(&self, notification: &Notification, agent: &ureq::Agent) -> Result<()>{
        let mut embed = serde_json::json!({
            "description": &notification.message,
            "color": 0x66ffcc
        });

        if notification.image.is_some() {
            embed["image"] = serde_json::json!({ "url": "attachment://cover.jpg" });
        }

        let payload = serde_json::json!({ "embeds": [embed] });
        let boundry = "knotify_boundry";

        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(format!(
            "--{boundry}\r\nContent-Disposition: form-data; name=\"payload_json\"\r\n\r\n{payload}\r\n"
        ).as_bytes());

        if let Some(img) = &notification.image {
            body.extend_from_slice(format!(
                "--{boundry}\r\nContent-Disposition: form-data; name=\"files[0]\"; filename=\"cover.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n"
            ).as_bytes());
            body.extend_from_slice(img);
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(format!("--{boundry}--\r\n").as_bytes());

        agent.post(&self.webhook_url)
            .set("Content-type", &format!("multipart/form-data; boundary={boundry}"))
            .send_bytes(&body)?;

        Ok(())
    }
}