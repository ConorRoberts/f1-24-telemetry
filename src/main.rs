mod db;
mod f1_telemetry_api;
mod f1_telemetry_client;

use clap::Parser;
// use db::connect_db;
use f1_telemetry_api::F1TelemetryApi;
// use f1_telemetry_client::{F1TelemetryClient, TelemetryPacket};
// use futures::StreamExt;
use std::error::Error;
// use std::process::exit;
use std::sync::Arc;
// use tokio::net::TcpListener;
// use tokio_tungstenite::{accept_async, WebSocketStream};
use tracing::Level;

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
        .with_max_level(if args.debug {
            Level::DEBUG
        } else {
            Level::ERROR
        })
        .init();

    // let db = connect_db().await.unwrap();

    // db.execute_batch(
    //     r#"
    //         create table if not exists car_telemetry (
    //             session_id text,
    //             timestamp real,
    //             throttle real,
    //             speed integer,
    //             primary key (session_id, timestamp)
    //         );
    //     "#,
    // )
    // .await?;

    // let udp_addr = format!("{}:{}", args.host, args.udp_port);
    let http_addr = format!("{}:{}", args.host, args.api_port);

    let api = F1TelemetryApi::new();
    let api_handle = Arc::new(api);

    api_handle.start(&http_addr).await?;

    Ok(())
}
