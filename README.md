# knotify

Kurotsuba's Streamer Notifier - A lightweight CLI tool that monitors streaming platforms and sends notifications when streamers go live.

## Features

- **Multi-platform monitoring**: YouTube, Bilibili
- **Notification support**: Discord webhooks with embedded cover images
- **No API keys required**: YouTube uses web scraping, Bilibili uses public API
- **Flexible runtime**: Run as a daemon (polling loop) or one-shot (cron-friendly)
- **Deduplication**: File-backed state tracking to avoid duplicate notifications
- **Modular architecture**: Easily extensible with new platforms and notifiers via traits

## Usage

```bash
# Daemon mode (polls every poll_interval_secs)
knotify --config config.yaml

# One-shot mode (check once and exit, cron-friendly)
knotify --config config.yaml --once
```

## Configuration

Create a `config.yaml`:

```yaml
poll_interval_secs: 300
state_file: "./knotify_state.json"

channels:
  - platform: youtube
    channel_id: "@ShirakamiFubuki"           # YouTube handle (recommended)
    name: "白上フブキ"                        # Set the name as you wish, in any UTF-8 charactor

  - platform: youtube
    channel_id: "UCxxxxxxxxxxxxxxxxxxxx" # or YouTube channel ID
    name: "StreamerName"

  - platform: bilibili
    channel_id: "12345"                  # room_id from live.bilibili.com/{room_id}
    name: "StreamerName"

notifiers:
  - type: discord
    endpoint: "https://discord.com/api/webhooks/..."
```

### Platform notes

| Platform | `channel_id` format | API key required |
|----------|---------------------|-----------------|
| YouTube  | `@Handle` or Channel ID (`UC...`) | No |
| Bilibili | Room ID (number from live URL) | No |

### Getting a YouTube channel identifier

Use either format in `channel_id`:
- **Handle** (recommended): `@ChannelHandle` - found in the channel URL or the homepage of the channel
- **Channel ID**: `UCxxxxxxxxxxxxxxxxxxxx` - found in the channel page source or via YouTube API

### Getting a Discord webhook URL

1. Right-click a text channel > Edit Channel
2. Go to Integrations > Webhooks > Create Webhook
3. Copy the webhook URL

### Notification format

When a streamer goes live, knotify sends a Discord embed with:

```
【直播提醒】
{channel_name}开播啦
{stream_title}
直播间地址：{stream_url}
```

The stream cover image is attached as an embedded image when available.

TODO: Add compatibility to customed templates.

## Building

```bash
cargo build --release
```

Pre-built binaries for Linux and Windows are available from [GitHub Actions](../../actions) workflow runs.

## TODO

- [ ] Add video post monitoring
- [ ] Add text post monitoring

## License

MIT
