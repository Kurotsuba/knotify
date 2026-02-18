use anyhow::Result;
use crate::types::Notification;

pub mod discord;

pub trait Notifier : Send + Sync {
    fn name(&self) -> &str;
    fn notify(&self, notification: &Notification) -> Result<()>;
}