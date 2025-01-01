use crate::f1_telemetry_client::{F1TelemetryClient, TelemetryPacket};
use futures_util::{stream::BoxStream, StreamExt};
use poem_openapi::{payload::EventStream, Enum, Object, OpenApi, Union};
use serde::{Deserialize, Serialize};
use std::{process::exit, sync::Arc};
use tokio::sync::broadcast;
use tracing::{debug, error};

// #[derive(Object, Clone)]
// struct EventMetadata {
//     timestamp: i64,
//     #[oai(skip)]
//     id: Option<u64>,
// }

// Data event
#[derive(Object, Clone)]
struct CarTelemetryEvent {
    #[oai(rename = "type")]
    event_type: EventType,
    throttle: f32,
    brake: f32,
    // #[oai(flatten)]
    // metadata: EventMetadata,
}

// Heartbeat event
#[derive(Object, Clone)]
struct HeartbeatEvent {
    #[oai(rename = "type")]
    event_type: EventType,
    // #[oai(flatten)]
    // metadata: EventMetadata,
}

#[derive(Clone, Enum, Serialize, Deserialize)]
#[oai(rename = "EventType")]
pub enum EventType {
    #[oai(rename = "data")]
    CarTelemetryEvent,
    #[oai(rename = "heartbeat")]
    Heartbeat,
}

// Union type for all possible events
#[derive(Union, Clone)]
#[oai(discriminator_name = "type", one_of)]
enum Event {
    #[oai(mapping = "data")]
    CarTelemetry(CarTelemetryEvent),
    #[oai(mapping = "heartbeat")]
    Heartbeat(HeartbeatEvent),
}

impl TryFrom<TelemetryPacket> for Event {
    type Error = String;

    fn try_from(value: TelemetryPacket) -> Result<Event, Self::Error> {
        match value {
            TelemetryPacket::CarTelemetry((_, data)) => {
                Ok(Event::CarTelemetry(CarTelemetryEvent {
                    event_type: EventType::CarTelemetryEvent,
                    brake: data.brake,
                    throttle: data.throttle,
                }))
            }
            _ => Err("Unimplemented".into()),
        }
    }
}

pub struct EventsApi {
    sender: broadcast::Sender<Event>,
}

#[OpenApi]
impl EventsApi {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);

        EventsApi { sender }
    }

    pub async fn start_listener(&self, addr: &str) {
        let client = F1TelemetryClient::new(addr).await.unwrap();

        let client_handle = Arc::new(client);

        let client_clone = client_handle.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            debug!("\nStopping telemetry capture...");

            client_clone.stop().await;

            exit(0);
        });

        client_handle
            .start(|x| {
                if let Ok(ev) = Event::try_from(x) {
                    if let Err(_) = self.sender.send(ev) {
                        error!("Error sending event");
                    }
                }
            })
            .await
            .unwrap()
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