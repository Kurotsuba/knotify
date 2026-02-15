use async_trait::async_trait;
use anyhow::Result;
use crate::types::StreamInfo;
use crate::config::ChannelConfig;

pub mod youtube;
pub mod bilibili;

#[async_trait]
pub trait Platform: Send + Sync {
    fn name(&self) -> &str;
    async fn check_live(&self, channel: &ChannelConfig) -> Result<Option<StreamInfo>>; 
}
