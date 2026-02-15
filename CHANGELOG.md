# Changelog

## 0.2.0

### Features

- **YouTube web scraping**: Replaced YouTube Data API v3 with web scraping — no API key required
- **YouTube @Handle support**: `channel_id` now accepts `@Handle` format in addition to `UCxxxx`
- **Cover image attachments**: Notifications include stream cover image as embedded attachment
- **Bilibili platform**: Monitor Bilibili live rooms via public API (no API key required)
- **Decoupled notifications**: Message construction separated from notification delivery

### Changes

- Removed `youtube_api_key` from config (no longer needed)
- `Notifier` trait now receives `Notification` struct instead of `StreamInfo`
- Discord notifier uses multipart form upload for cover images

## 0.1.0

### Features

- **YouTube platform**: Monitor YouTube channels for live streams via YouTube Data API v3
- **Discord notifier**: Send notifications via Discord webhooks with rich embeds
- **Deduplication**: File-backed JSON state tracking prevents duplicate notifications
- **Daemon mode**: Continuous polling with configurable interval (`poll_interval_secs`)
- **One-shot mode**: `--once` flag for single check cycle (cron-friendly)
- **YAML configuration**: Simple config file for channels, notifiers, and runtime settings
- **GitHub Actions CI**: Automated release builds for Linux and Windows

### Architecture

- Trait-based platform system (`Platform` trait) for easy extension
- Trait-based notifier system (`Notifier` trait) for easy extension
- Modular file structure with separate modules for platforms, notifiers, config, and state
