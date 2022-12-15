#![crate_name = "wunder"]
#![deny(missing_docs)]

//! # wunder
//! 
//! A CLI tool to interact with the Mirakl API

#[macro_use] extern crate serde_with_macros;

use std::{fs::{File, OpenOptions}, sync::Arc, path::{PathBuf, Path}};

use clap::{Parser, Subcommand};
use miette::IntoDiagnostic;
use models::tracking::{TrackingRequest, XmlTrackingRequest};
use reqwest::Body;
use tokio_util::codec::{FramedRead, BytesCodec};
use tracing::{info, debug};
use tracing_subscriber::{prelude::*, filter};
use std::io::Write;

use crate::{prelude::*, models::invoices::{OrderDocuments, OrderDocument}};

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
        tracking_file: String,
    },
    PushInvoice {
        invoice_file: String,
    },
}

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const DOCUMENT_FORMATS: [&str; 17] = [
    "csv",
    "doc",
    "xls",
    "xlsx",
    "ppt",
    "pdf",
    "odt",
    "ods",
    "odp",
    "txt",
    "rtf",
    "png",
    "jpg",
    "gif",
    "zpl",
    "mov",
    "mp4",
];

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
    debug!("Pushing tracking info from {}", tracking_file.as_ref().display());
    let buf_reader = std::io::BufReader::new(File::open(tracking_file)?);
    let tracking_request: XmlTrackingRequest = quick_xml::de::from_reader(buf_reader)?;
    let order_id = tracking_request.order_id.clone();
    let client = reqwest::Client::new();
    let response = client.post(format!("{}/api/orders/{}/tracking", &config.base_url, &tracking_request.order_id))
        .header("Authorization", &config.api_key)
        .header("User-Agent", USER_AGENT)
        .json(&Into::<TrackingRequest>::into(tracking_request))
        .send()
        .await?
        .error_for_status()?;
    debug!("Tracking push response: {}", response.text().await?);
    info!("Pushed tracking info for order {}, verifying...", &order_id);
    client.get(format!("{}/api/orders/{}/ship", &config.base_url, &order_id))
        .header("Authorization", &config.api_key)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .error_for_status()?;
    info!("Verified tracking info for order {}", &order_id);

    Ok(())
}

async fn push_invoice<P>(config: &crate::config::Config, tracking_file: P) -> Result<()> 
where
    P: AsRef<Path>
{
    debug!("Pushing invoice from {}", tracking_file.as_ref().display());
    let file_name = tracking_file.as_ref().file_name().unwrap().to_str().unwrap();
    let extension = tracking_file.as_ref().extension().unwrap().to_str().unwrap();
    let order_id = file_name.split('_').next().unwrap();
    let file = tokio::fs::File::open(&tracking_file).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    let upload_file_name = format!("Invoice-{}.{}", order_id, extension);

    let document_info = OrderDocuments {
        order_documents: vec![
            OrderDocument {
                file_name: upload_file_name.clone(),
                type_code: "CUSTOMER_INVOICE".to_string(),
            }
        ],
    };
    let json = serde_json::to_string(&document_info)?;
    let json_bytes = json.as_bytes().to_vec();

    let client = reqwest::Client::new();
    let response = client.post(format!("{}/api/orders/{}/documents", &config.base_url, order_id))
        .header("Authorization", &config.api_key)
        .header("User-Agent", USER_AGENT)
        .multipart(
            // why does it have to be multipart? :(
            reqwest::multipart::Form::new()
                .part(
                    "files", 
                    reqwest::multipart::Part::stream(file_body)
                        .file_name(upload_file_name)
                )
                // is this correct? find out on the next episode
                .part(
                    "order_documents",
                    reqwest::multipart::Part::bytes(json_bytes)
                        .mime_str("application/json")?
                )

        )
        .send()
        .await?
        .error_for_status()?
        .json::<models::invoices::OrderDocumentResponse>()
        .await?;
    
    // ugly
    if let Some(error_count) = response.errors_count {
        if error_count > 0 {
            return Err(crate::error::WunderError::DocumentUploadError(response));
        }
    }
    info!("Pushed invoice for order {}", &order_id);

    todo!()
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
            let tracking_path = PathBuf::from(&tracking_file);
            if !tracking_path.exists() {
                return Err(crate::error::WunderError::TrackingFileNotFound(tracking_file));
            }

            push_tracking_info(&config, tracking_path).await
        },
        CliSubcommand::PushInvoice { invoice_file } => {
            info!("Pushing invoice");
            let invoice_path = PathBuf::from(&invoice_file);
            if !invoice_path.exists() {
                return Err(crate::error::WunderError::InvoiceFileNotFound(invoice_file));
            }

            let file_ext = invoice_path.extension().unwrap().to_str().unwrap().to_lowercase();
            if !DOCUMENT_FORMATS.contains(&file_ext.as_str()) {
                return Err(crate::error::WunderError::DocumentFormatNotSupported(file_ext.to_string()));
            }

            push_invoice(&config, invoice_path).await
        }
    };
    if let Err(e) = result {
        tracing::error!("{}", e);
        return Err(e);
    }

    Ok(())
}
