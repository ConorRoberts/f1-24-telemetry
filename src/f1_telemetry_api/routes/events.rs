use crate::f1_telemetry_api::events::Event;
use crate::f1_telemetry_api::events::LapDataEvent;
use crate::f1_telemetry_client::F1TelemetryClient;
use futures_util::{stream::BoxStream, StreamExt};
use poem::web::Query;
use poem::Result;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::ApiResponse;
use poem_openapi::{payload::EventStream, OpenApi};
use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::{debug, error};

struct PositionEvent {
    x: f32,
    y: f32,
    speed: u16,
}

pub struct EventsApi {
    sender: Arc<broadcast::Sender<Event>>,
    data: Arc<Mutex<Vec<Event>>>,
}

#[derive(ApiResponse)]
enum GetLapDataResponse {
    #[oai(status = 200)]
    Success(Json<Vec<LapDataEvent>>),
}

#[OpenApi]
impl EventsApi {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);

        EventsApi {
            sender: Arc::new(sender),
            data: Arc::new(Mutex::new(Vec::new())),
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
        let data_clone = self.data.clone();

        // Listen for events on the telemetry client and
        // 1. send them in realtime to all listeners
        // 2. save them in memory for further processing
        tokio::spawn(async move {
            let lap_number: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
            client_clone
                .start(|x| {
                    if let Ok(ev) = Event::try_from(x) {
                        let ev_clone = ev.clone();
                        if let Event::CarMotion(_) = ev {
                            if let Err(e) = sender.send(ev) {
                                error!("Error sending event {:?}", e.0);
                            }
                        }

                        let mut g = lap_number.try_lock().unwrap();
                        if let Event::LapData(lap_data) = ev_clone.clone() {
                            (*g) = lap_data.current_lap_num.into();
                        }

                        if matches!(
                            ev_clone,
                            Event::CarMotion(_) | Event::LapData(_) | Event::CarTelemetry(_)
                        ) {
                            let mut data = data_clone.lock().unwrap();
                            (*data).push(ev_clone);
                        }
                    }
                })
                .await
                .unwrap()
        });
    }

    /// SSE for real-time telemetry
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

    #[oai(path = "/get_lap_data", method = "get")]
    async fn get_lap_data(&self, start_time: Query<Option<String>>) -> Result<GetLapDataResponse> {
        let data = self.data.try_lock().unwrap();

        let arr: Vec<LapDataEvent> = data
            .iter()
            .filter_map(|x| match x {
                Event::LapData(d) => Some(d.clone()),
                _ => None,
            })
            .collect();

        return Ok(GetLapDataResponse::Success(Json(arr)));
    }
}
