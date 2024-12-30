mod packets;

use chrono::{DateTime, Utc};
use libsql::params;
use packets::car_motion_data::PacketMotionData;
use packets::car_telemetry::PacketCarTelemetry;
use packets::header::PacketHeader;
use packets::lap_data::PacketLapData;
use packets::session_data::PacketSessionData;
use packets::{header::PacketType, Packet};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};
use tracing::{debug, error, info};

use crate::db::connect_db;

type SessionData = HashMap<DateTime<Utc>, PacketCarTelemetry>;

pub struct F1TelemetryClient {
    socket: Arc<UdpSocket>,
    session_data: Arc<Mutex<SessionData>>,
    running: Arc<Mutex<bool>>,
    data: Arc<Mutex<Vec<(PacketHeader, FlushData)>>>,
}

enum FlushData {
    Session(PacketSessionData),
    Motion(PacketMotionData),
    CarTelemetry(PacketCarTelemetry),
    LapData(PacketLapData),
}

impl F1TelemetryClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket: Arc::new(socket),
            session_data: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(true)),
            data: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        println!("Listening for F1 24 telemetry data...");
        let mut buf = [0u8; 2048];

        self.start_periodic_flush().await;

        while *self.running.lock().await {
            match self.socket.recv(&mut buf).await {
                Ok(size) => {
                    if let Err(e) = self.process_packet(&buf[..size]).await {
                        error!("Error processing packet: {}", e);
                    }
                }
                Err(e) => error!("Error receiving data: {}", e),
            }
        }
        Ok(())
    }

    async fn start_periodic_flush(&self) {
        let data = self.data.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(500));

            loop {
                interval.tick().await;

                // Lock and swap the vector with an empty one
                let data_to_flush = {
                    let mut d = data.lock().await;
                    std::mem::take(&mut *d) // Takes ownership and leaves an empty vec
                };

                if !data_to_flush.is_empty() {
                    info!("Saving");
                    let db = connect_db().await.unwrap();

                    if let Ok(tx) = db.transaction().await {
                        for packet in data_to_flush {
                            match packet.1 {
                                FlushData::CarTelemetry(x) => {
                                    info!("Session ID: {}", packet.0.session_uid);
                                    tx.execute("insert into car_telemetry (session_id,speed,throttle,timestamp) values (?1,?2,?3,?4);", params![packet.0.session_uid.to_string(),x.speed,x.throttle,packet.0.session_time]).await.unwrap();
                                    ()
                                }
                                _ => (),
                            }
                        }

                        if let Err(e) = tx.commit().await {
                            error!("Error saving data: {}", e);
                        }
                    };

                    // match db.save_batch(&data_to_flush).await {
                    //     Ok(_) => {
                    //         info!("Flushed {} records to database", data_to_flush.len());
                    //     }
                    //     Err(e) => {
                    //         error!("Failed to flush to database: {}", e);
                    //         // You might want to retry or handle the error differently
                    //     }
                    // }
                }
            }
        });
    }

    async fn queue_packet(&self, header: PacketHeader, packet: FlushData) {
        let mut data = self.data.lock().await;
        data.push((header, packet));
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    async fn process_packet(&self, data: &[u8]) -> Result<(), String> {
        let header = PacketHeader::try_from(data)?;

        // debug!(
        //     "Received packet \"{:?}\" of size {}",
        //     header.packet_id,
        //     data.len() - PacketHeader::size()
        // );

        let bytes = &data[PacketHeader::size()..];

        let p: Option<(PacketHeader, FlushData)> = match header.packet_id {
            PacketType::Motion => Some((
                header,
                FlushData::Motion(PacketMotionData::try_from(bytes)?),
            )),
            PacketType::Session => Some((
                header,
                FlushData::Session(PacketSessionData::try_from(bytes)?),
            )),
            PacketType::CarTelemetry => Some((
                header,
                FlushData::CarTelemetry(PacketCarTelemetry::try_from(bytes)?),
            )),
            PacketType::LapData => {
                Some((header, FlushData::LapData(PacketLapData::try_from(bytes)?)))
            }
            _ => None,
        };

        if let Some(data) = p {
            self.queue_packet(data.0, data.1).await
        }

        Ok(())
    }
}
