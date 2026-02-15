# knotify

Kurotsuba's Streamer Notifier - A lightweight CLI tool that monitors streaming platforms and sends notifications when streamers go live.

## Features

- **Multi-platform monitoring**: YouTube, Bilibili
- **Notification support**: Discord webhooks
- **Flexible runtime**: Run as a daemon (polling loop) or one-shot (cron-friendly)
- **Deduplication**: Tracks live status to avoid duplicate notifications
- **Modular architecture**: Easily extensible with new platforms and notifiers via traits

## Usage

```bash
# Daemon mode (polls every poll_interval_secs)
knotify --config config.yaml

# One-shot mode (check once and exit)
knotify --config config.yaml --once
```

## Configuration

Create a `config.yaml`:

```yaml
poll_interval_secs: 300
state_file: "./knotify_state.json"

# Required only if monitoring YouTube channels
youtube_api_key: "YOUR_YOUTUBE_API_KEY"

channels:
  - platform: youtube
    channel_id: "UCxxxxxxxxxxxxxxxxxxxx"
    name: "StreamerName"

  - platform: bilibili
    channel_id: "12345"   # room_id from live.bilibili.com/{room_id}
    name: "StreamerName"

notifiers:
  - type: discord
    endpoint: "https://discord.com/api/webhooks/..."
```

### Platform notes

| Platform | `channel_id` | API key required |
|----------|-------------|-----------------|
| YouTube  | Channel ID (starts with `UC`) | Yes (`youtube_api_key`) |
| Bilibili | Room ID (number from live URL) | No |

### Getting a YouTube API key

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a project and enable **YouTube Data API v3**
3. Create an API key under **Credentials**

Note: The free tier allows ~100 checks/day (each search costs 100 quota units out of 10,000/day). Adjust `poll_interval_secs` accordingly.

### Getting a YouTube Channel ID
1. Get your YouTube API key
2. Run following command, streamer_name should be something like "@ShirakamiFubuki"
```
 curl "https://www.googleapis.com/youtube/v3/channels?part=id&forHandle=<streamer_name>&key=<your_api_key>"
```
3. Get items[0]["id"], a string starts with "UC".

### Getting a Discord webhook URL

1. Right-click a text channel > Edit Channel
2. Go to Integrations > Webhooks > Create Webhook
3. Copy the webhook URL

## Building

```bash
cargo build --release
```

## TODO

- [ ] Add video post monitoring
- [ ] Add text post monitoring

## License

MIT
