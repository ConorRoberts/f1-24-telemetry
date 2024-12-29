mod f1_telemetry_client;

use f1_telemetry_client::F1TelemetryClient;
use std::error::Error;
use std::process::exit;
use std::sync::Arc;

use clap::Parser;

/// F1 24 Telemetry Client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// UDP port to listen on
    #[arg(short, long, default_value_t = 20777)]
    port: u16,

    /// IP address to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.address, args.port);
    let client = F1TelemetryClient::new(&addr).await?;
    let client_handle = Arc::new(client);

    // Set up Ctrl+C handler
    let client_clone = client_handle.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\nStopping telemetry capture...");
        client_clone.stop().await;

        // Save session data before exiting
        let filename = format!(
            "f1_telemetry_{}.json",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        if let Err(e) = client_clone.save_session_data(&filename).await {
            eprintln!("Error saving session data: {}", e);
        } else {
            println!("Session data saved to {}", filename);
        }

        exit(0);
    });

    // Start the client
    client_handle.start().await?;

    Ok(())
}
