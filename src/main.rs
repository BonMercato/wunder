#[macro_use] extern crate serde_with_macros;

use std::{fs::{File, OpenOptions}, sync::Arc, path::{PathBuf, Path}};

use clap::{Parser, Subcommand};
use miette::IntoDiagnostic;
use tracing::{info, debug};
use tracing_subscriber::{prelude::*, filter};
use std::io::Write;

use crate::prelude::*;

mod models;
mod prelude;
mod config;
mod error;

#[derive(Debug, Parser)]
#[command(about, version, author, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    pub command: CliSubcommand
}

#[derive(Debug, Subcommand)]
enum CliSubcommand {
    PullOrders,
    PushTrackingInfo {
        tracking_file: String
    },
}

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn initialize_logging() -> Result<()> {
    let stdout_log = tracing_subscriber::fmt::layer();
    
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("wunder.log")?;
    let file_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(stdout_log)
        .with(file_log)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    Ok(())
}

async fn pull_orders(config: &crate::config::Config) -> Result<()> {
    let order_path = PathBuf::from(&config.pull_order_settings.order_path);
    if !order_path.exists() {
        tokio::fs::create_dir_all(&order_path).await?;
    }

    let client = reqwest::Client::new();
    debug!("Fetching orders that have the following state codes: {}", config.pull_order_settings.order_state_codes());
    let mut url = Some(format!("{}/api/orders", &config.base_url));
    
    while url.is_some() {
        let response = client.get(url.as_ref().unwrap())
            .header("Authorization", &config.api_key)
            .header("User-Agent", USER_AGENT)
            .query(&[("order_state_codes", config.pull_order_settings.order_state_codes())])
            .send()
            .await?
            .error_for_status()?;
        url = response.headers()
            .get(reqwest::header::LINK)
            .map(|l| l.to_str().unwrap().to_string());
        let order_response = response.json::<models::orders::OrderResponse>().await?;
        debug!("Fetched {} orders", order_response.total_count);
        for order in order_response.orders {
            if &order.order_state == "WAITING_ACCEPTANCE" {
                client.put(format!("{}/api/orders/{}/accept", &config.base_url, order.order_id))
                    .header("Authorization", &config.api_key)
                    .header("User-Agent", USER_AGENT)
                    .send()
                    .await?
                    .error_for_status()?;
            }

            let order_file = order_path.join(format!("{}-{}-GetOrders_Response.xml", chrono::Local::now().format("%Y%m%d-%H%M%S"), order.order_id));
            let mut file = File::create(&order_file)?;
            let order_xml = quick_xml::se::to_string(&order)?;
            file.write_all(order_xml.as_bytes())?;
            info!("Wrote order {} to {}", order.order_id, order_file.display());
        }
    }

    Ok(())
}

async fn push_tracking_info<P>(config: &crate::config::Config, tracking_file: P) -> Result<()>
where
    P: AsRef<Path>
{
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    initialize_logging()?;
    let config = config::Config::load()?;

    let result = match args.command {
        CliSubcommand::PullOrders => {
            info!("Pulling orders");
            pull_orders(&config).await
        },
        CliSubcommand::PushTrackingInfo { tracking_file } => {
            info!("Pushing tracking info");
            let path = PathBuf::from(&tracking_file);
            if !path.exists() {
                return Err(crate::error::WunderError::TrackingFileNotFound(tracking_file));
            }

            push_tracking_info(&config, path).await
        }
    };
    if let Err(e) = result {
        tracing::error!("{}", e);
        return Err(e);
    }

    Ok(())
}
