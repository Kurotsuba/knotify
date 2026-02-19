use std::io::Read;

use anyhow::Result;

use crate::platform::Platform;
use crate::config::ChannelConfig;
use crate::types::StreamInfo;

pub struct YouTubePlatform;

impl YouTubePlatform {
    pub fn new() -> Self {
        YouTubePlatform {}
    }
}

impl Platform for YouTubePlatform {
    fn name(&self) -> &str {
        "youtube"
    }

    fn check_live(&self, channel: &ChannelConfig, agent: &ureq::Agent) -> Result<Option<StreamInfo>> {
        let url = if channel.channel_id.starts_with('@') {
            format!("https://www.youtube.com/{}/live", channel.channel_id)
        } else {
            format!("https://www.youtube.com/channel/{}/live", channel.channel_id)
        };

        let reader = agent.get(&url)
            .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .call()?
            .into_reader();

        let json_str = match extract_json_streaming(reader, b"var ytInitialPlayerResponse = ") {
            Some(s) => s,
            None => return Ok(None),
        };

        let data: serde_json::Value = serde_json::from_str(&json_str)?;
        let details = &data["videoDetails"];

        let is_live = details["isLive"].as_bool().unwrap_or(false);
        if !is_live {
            return Ok(None);
        }

        let video_id = details["videoId"].as_str().unwrap_or_default();
        let title = details["title"].as_str().unwrap_or_default();
        let author = details["author"].as_str().unwrap_or(&channel.name);

        let cover = details["thumbnail"]["thumbnails"]
            .as_array()
            .and_then(|arr| arr.last())
            .and_then(|t| t["url"].as_str())
            .map(|s| s.to_string());

        Ok(Some(StreamInfo {
            platform: "youtube".to_string(),
            channel_id: channel.channel_id.clone(),
            channel_name: author.to_string(),
            title: title.to_string(),
            url: format!("https://www.youtube.com/watch?v={}", video_id),
            cover,
        }))
    }
}

fn extract_json_streaming<R: Read>(mut reader: R, prefix: &[u8]) -> Option<String> {
    const CHUNK: usize = 4096;
    let plen = prefix.len();
    let mut scan_buf: Vec<u8> = Vec::new();
    let mut json_buf: Vec<u8> = Vec::new();
    let mut in_json = false;
    let mut depth: i32 = 0;

    loop {
        let mut chunk = vec![0u8; CHUNK];
        let n = reader.read(&mut chunk).ok()?;
        if n == 0 { return None; }
        chunk.truncate(n);

        if !in_json {
            scan_buf.extend_from_slice(&chunk);
            if let Some(pos) = scan_buf.windows(plen).position(|w| w == prefix) {
                let rest: Vec<u8> = scan_buf[pos + plen..].to_vec();
                scan_buf = Vec::new();
                in_json = true;
                for &b in &rest {
                    json_buf.push(b);
                    match b {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                return std::str::from_utf8(&json_buf).ok().map(str::to_string);
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                // Keep last (plen-1) bytes as overlap to detect prefix spanning chunks
                if scan_buf.len() > plen - 1 {
                    let trim = scan_buf.len() - (plen - 1);
                    scan_buf.drain(..trim);
                }
            }
        } else {
            for &b in &chunk {
                json_buf.push(b);
                match b {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return std::str::from_utf8(&json_buf).ok().map(str::to_string);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
