use async_trait::async_trait;
use anyhow::Result;
use crate::types::Notification;

pub mod discord;

#[async_trait]
pub trait Notifier : Send + Sync {
    fn name(&self) -> &str;
    async fn notify(&self, notification: &Notification) -> Result<()>;
}