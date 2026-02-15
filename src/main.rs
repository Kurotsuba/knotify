mod types;
mod config;
mod platform;
mod notifier;
mod status;

use clap::Parser;
use std::{collections::HashMap, time::Duration};
use crate::{notifier::Notifier, platform::Platform};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(long)]
    once: bool,
}

async fn check_and_notify (
    platforms: &HashMap<String, Box<dyn Platform>>,
    notifiers: &[Box<dyn Notifier>],
    channels: &[config::ChannelConfig],
    live_status: &mut status::LiveStatus,
) {
    for chn in channels {
        let Some(platform) = platforms.get(&chn.platform) else {
            println!("Unknown platform: {}", chn.platform);
            continue;
        };

        let result = match platform.check_live(chn).await {
            Ok(r) => r,
            Err(e) => {println!("Check error: {}", e); continue;},
        };

        if !live_status.should_notify(&chn.platform, &chn.channel_id, result.is_some()) {
            continue;
        }

        let Some(sinfo) = result else {continue;};

        let image = match &sinfo.cover {
            Some(url) => match reqwest::get(url).await {
                Ok(resp) => resp.bytes().await.ok().map(|b| b.to_vec()),
                Err(e) => { println!("Failed to fetch cover image: {}", e); None }
            },
            None => None,
        };

        let notification = types::Notification {
            message: format!(
                "【直播提醒】\n{}开播啦\n{}\n直播间地址：{}",
                sinfo.channel_name, sinfo.title, sinfo.url
            ),
            image,
        };

        for ntf in notifiers {
            if let Err(e) = ntf.notify(&notification).await {
                println!("Notify error {}: {}", ntf.name(), e);
            }
        }

    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let app_config = config::load_config(&cli.config).expect("Fail loading config");
    let mut live_status = status::LiveStatus::load_status(&app_config.state_file);

    let notifiers: Vec<Box<dyn Notifier>> = app_config.notifiers.iter()
        .filter_map(|n| match n.notifier_type.as_str() {
            "discord" => Some(Box::new(
                notifier::discord::DiscordNotifier::new(n.endpoint.clone().expect("Missing endpoint"))
            ) as Box<dyn Notifier>),
            _ => {
                println!("Unknown notifier: {}", n.notifier_type);
                None
            }
        })
        .collect();

    let mut platforms: HashMap<String, Box<dyn Platform>> = HashMap::new();
    for chn in &app_config.channels {
        if platforms.contains_key(&chn.platform) {
            continue;
        }
        match chn.platform.as_str() {
            "youtube" => {
                let api_key = app_config.youtube_api_key.clone()
                    .expect("youtube_api_key required for YouTube channels");
                platforms.insert("youtube".into(), Box::new(
                    platform::youtube::YouTubePlatform::new(api_key)
                ));
            }
            "bilibili" => {
                platforms.insert("bilibili".into(), Box::new(
                    platform::bilibili::BilibiliPlatform::new()
                ));
            }
            other => println!("Unknown platform: {}", other),
        }
    }

    loop {
        check_and_notify(&platforms, &notifiers, &app_config.channels, &mut live_status).await;
        live_status.save_status(&app_config.state_file).ok();

        if cli.once { break; }
        tokio::time::sleep(Duration::from_secs(app_config.poll_interval_secs as u64)).await;        
    }
}
