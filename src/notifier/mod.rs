use async_trait::async_trait;
use anyhow::Result;
use crate::types::StreamInfo;

pub mod discord;

#[async_trait]
pub trait Notifier : Send + Sync {
    fn name(&self) -> &str;
    async fn notify(&self, info: &StreamInfo) -> Result<()>;
}