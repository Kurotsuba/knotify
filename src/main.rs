mod types;
mod config;
mod platform;
mod notifier;
mod status;

use std::{collections::HashMap, io::Read, time::Duration};
use crate::{notifier::Notifier, platform::Platform};

struct Cli {
    config: String,
    once: bool,
}

impl Cli {
    fn parse() -> Self {
        let mut config = "config.toml".to_string();
        let mut once = false;
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--once" => once = true,
                "--config" | "-c" => {
                    config = args.next().expect("--config requires a value");
                }
                other => eprintln!("Unknown argument: {}", other),
            }
        }
        Cli { config, once }
    }
}


fn check_and_notify(
    agent: &ureq::Agent,
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

        let result = match platform.check_live(chn, agent) {
            Ok(r) => r,
            Err(e) => {println!("Check error: {}", e); continue;},
        };

        if !live_status.should_notify(&chn.platform, &chn.channel_id, result.is_some()) {
            continue;
        }

        let Some(sinfo) = result else {continue;};

        let image = sinfo.cover.as_ref().and_then(|url| {
            let mut buf = Vec::new();
            agent.get(url).call().ok()?.into_reader().read_to_end(&mut buf).ok()?;
            Some(buf)
        });

        let notification = types::Notification {
            message: format!(
                "【直播提醒】\n{}开播啦\n{}\n直播间地址：{}",
                chn.name, sinfo.title, sinfo.url
            ),
            image,
        };

        for ntf in notifiers {
            if let Err(e) = ntf.notify(&notification, agent) {
                println!("Notify error {}: {}", ntf.name(), e);
            }
        }
    }
}

fn main() {
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
                platforms.insert("youtube".into(), Box::new(
                    platform::youtube::YouTubePlatform::new()
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

    let agent = ureq::Agent::new();

    loop {
        check_and_notify(&agent, &platforms, &notifiers, &app_config.channels, &mut live_status);
        live_status.save_status(&app_config.state_file).ok();

        if cli.once { break; }
        std::thread::sleep(Duration::from_secs(app_config.poll_interval_secs as u64));
    }
}
