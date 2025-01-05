use crate::f1_telemetry_api::events::Event;
use crate::f1_telemetry_client::F1TelemetryClient;
use futures_util::{stream::BoxStream, StreamExt};
use poem_openapi::{payload::EventStream, OpenApi};
use std::{process::exit, sync::Arc};
use tokio::sync::broadcast;
use tracing::{debug, error};

pub struct EventsApi {
    sender: Arc<broadcast::Sender<Event>>,
}

#[OpenApi]
impl EventsApi {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);

        EventsApi {
            sender: Arc::new(sender),
        }
    }

    pub async fn start_listener(&self, addr: &str) {
        let client = F1TelemetryClient::new(addr).await.unwrap();

        let client_handle = Arc::new(client);

        let ctrlc_client_clone = client_handle.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            debug!("\nStopping telemetry capture...");

            ctrlc_client_clone.stop().await;

            exit(0);
        });

        let client_clone = client_handle.clone();
        let sender = self.sender.clone();

        tokio::spawn(async move {
            client_clone
                .start(|x| {
                    if let Ok(ev) = Event::try_from(x) {
                        if let Err(e) = sender.send(ev) {
                            error!("Error sending event {:?}", e.0);
                        }
                    }
                })
                .await
                .unwrap()
        });
    }

    #[oai(path = "/events", method = "get")]
    async fn index(&self) -> EventStream<BoxStream<'static, Event>> {
        // Create a new receiver
        let mut receiver = self.sender.subscribe();

        // Convert the receiver into a stream
        let stream = async_stream::stream! {
            while let Ok(event) = receiver.recv().await {
                yield event;
            }
        };

        EventStream::new(stream.boxed())
    }
}
