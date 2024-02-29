#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::{CombinedLogger, TermLogger, WriteLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let discord_webhook_url = dotenvy::var("DISCORD_WEBHOOK_URL")?;
    let checkip_url = dotenvy::var("CHECKIP_URL")?;
    let interval_min: u64 = dotenvy::var("INTERVAL_MIN")?.parse()?;
    let log_file_name = dotenvy::var("LOG_FILE_NAME")?;
    let mut current_ip = String::new();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_file_name)?)
        ]
    )?;

    let client = reqwest::Client::new();
    loop {
        match client.get(&checkip_url).send().await {
            Ok(res) => {
                let res_ip = res.text().await?;
                if res_ip != current_ip {
                    current_ip = res_ip;
                    info!("ip: {current_ip}");
                    let req_header = HashMap::from([
                        ("content", current_ip.trim())
                    ]);
                    match client.post(&discord_webhook_url).json(&req_header).send().await {
                        Ok(_) => {
                            info!("Post IP-address on Discord");
                        }
                        Err(err) => {
                            error!("{err}");
                        }
                    };
                }
            }
            Err(_) => {
                warn!("connection failed");
                current_ip = String::from("");
            }
        }
        sleep(Duration::from_secs(interval_min * 60)).await;
    }
}
