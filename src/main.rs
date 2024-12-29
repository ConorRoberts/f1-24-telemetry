mod f1_telemetry_api;
mod f1_telemetry_client;

use clap::Parser;
use f1_telemetry_api::F1TelemetryApi;
use f1_telemetry_client::F1TelemetryClient;
use std::error::Error;
use std::process::exit;
use std::sync::Arc;
use tracing::{debug, error, Level};

/// F1 24 Telemetry Client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// UDP port to listen on
    #[arg(long, default_value_t = 20777)]
    udp_port: u16,

    /// UDP port to listen on
    #[arg(long, default_value_t = 4000)]
    api_port: u16,

    /// IP address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(match args.debug {
            true => Level::DEBUG,
            _ => Level::ERROR,
        })
        .init();

    let udp_addr = format!("{}:{}", args.host, args.udp_port);
    let http_addr = format!("{}:{}", args.host, args.api_port);
    let client = F1TelemetryClient::new(&udp_addr).await?;
    let client_handle = Arc::new(client);

    let api = F1TelemetryApi::new();
    let api_handle = Arc::new(api);

    // Set up Ctrl+C handler
    let client_clone = client_handle.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        debug!("\nStopping telemetry capture...");

        client_clone.stop().await;

        exit(0);
    });

    // Start the client
    tokio::spawn(async move {
        client_handle.start().await.unwrap();
    });

    api_handle.start(&http_addr).await?;

    Ok(())
}
