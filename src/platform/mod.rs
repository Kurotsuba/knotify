use anyhow::Result;
use crate::types::StreamInfo;
use crate::config::ChannelConfig;

pub mod youtube;
pub mod bilibili;

pub trait Platform: Send + Sync {
    fn name(&self) -> &str;
    fn check_live(&self, channel: &ChannelConfig, agent: &ureq::Agent) -> Result<Option<StreamInfo>>;
}
