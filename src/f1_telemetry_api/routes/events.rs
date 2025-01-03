use crate::f1_telemetry_client::{F1TelemetryClient, TelemetryPacket};
use futures_util::{stream::BoxStream, StreamExt};
use poem_openapi::{payload::EventStream, Enum, Object, OpenApi, Union};
use serde::{Deserialize, Serialize};
use std::{any::Any, process::exit, sync::Arc};
use tokio::sync::broadcast;
use tracing::{debug, error, info};

// Data event
#[derive(Object, Clone, Debug)]
struct CarTelemetryEvent {
    #[oai(rename = "type")]
    event_type: EventType,
    throttle: f32,
    brake: f32,
    speed: u16,
    brake_temp: [u16; 4],       // Brake temperatures (FL, FR, RL, RR)
    tyre_surface_temp: [u8; 4], // Tyre surface temperatures
    tyre_inner_temp: [u8; 4],   // Tyre inner temperatures
    engine_temperature: u16,
    tyre_pressure: [f32; 4],
}

#[derive(Object, Clone, Debug)]
struct CarMotionEvent {
    #[oai(rename = "type")]
    event_type: EventType,
    world_position_x: f32,
    world_position_y: f32,
    world_position_z: f32,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
}

// Heartbeat event
#[derive(Object, Clone, Debug)]
struct HeartbeatEvent {
    #[oai(rename = "type")]
    event_type: EventType,
}

#[derive(Clone, Enum, Serialize, Deserialize, Debug)]
#[oai(rename = "EventType")]
pub enum EventType {
    #[oai(rename = "car_telemetry")]
    CarTelemetryEvent,
    #[oai(rename = "car_motion")]
    CarMotionEvent,
    #[oai(rename = "heartbeat")]
    Heartbeat,
}

// Union type for all possible events
#[derive(Union, Clone, Debug)]
#[oai(discriminator_name = "type", one_of)]
enum Event {
    #[oai(mapping = "car_telemetry")]
    CarTelemetry(CarTelemetryEvent),
    #[oai(mapping = "car_motion")]
    CarMotion(CarMotionEvent),
    #[oai(mapping = "heartbeat")]
    Heartbeat(HeartbeatEvent),
}

impl TryFrom<TelemetryPacket> for Event {
    type Error = String;

    fn try_from(value: TelemetryPacket) -> Result<Event, Self::Error> {
        match value {
            TelemetryPacket::CarTelemetry((_, data)) => {
                info!("{:?}", data.brake_temp);
                Ok(Event::CarTelemetry(CarTelemetryEvent {
                    event_type: EventType::CarTelemetryEvent,
                    brake: data.brake,
                    throttle: data.throttle,
                    speed: data.speed,
                    tyre_inner_temp: data.tyre_inner_temp,
                    tyre_surface_temp: data.tyre_surface_temp,
                    brake_temp: data.brake_temp,
                    engine_temperature: data.engine_temperature,
                    tyre_pressure: data.tyre_pressure,
                }))
            }
            TelemetryPacket::Motion((_, data)) => {
                if let Some(m) = data.car_motion_data.get(0) {
                    Ok(Event::CarMotion(CarMotionEvent {
                        event_type: EventType::CarTelemetryEvent,
                        g_force_lateral: m.g_force_lateral,
                        g_force_longitudinal: m.g_force_longitudinal,
                        g_force_vertical: m.g_force_vertical,
                        world_position_x: m.world_position_x,
                        world_position_y: m.world_position_y,
                        world_position_z: m.world_position_z,
                    }))
                } else {
                    Err("Could not get car data for first car".into())
                }
            }
            _ => Err("Unimplemented".into()),
        }
    }
}

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
