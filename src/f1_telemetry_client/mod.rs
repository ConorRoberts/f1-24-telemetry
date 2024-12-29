mod packets;

use chrono::{DateTime, Utc};
use packets::car_motion_data::PacketMotionData;
use packets::car_telemetry::PacketCarTelemetry;
use packets::header::PacketHeader;
use packets::lap_data::PacketLapData;
use packets::session_data::PacketSessionData;
use packets::{Packet, PacketType};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

type SessionData = HashMap<DateTime<Utc>, PacketCarTelemetry>;

pub struct F1TelemetryClient {
    socket: Arc<UdpSocket>,
    #[allow(dead_code)]
    session_data: Arc<Mutex<SessionData>>,
    running: Arc<Mutex<bool>>,
}

impl F1TelemetryClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket: Arc::new(socket),
            session_data: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(true)),
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        println!("Listening for F1 24 telemetry data...");
        let mut buf = [0u8; 2048];

        while *self.running.lock().await {
            match self.socket.recv(&mut buf).await {
                Ok(size) => {
                    if let Err(e) = self.process_packet(&buf[..size]).await {
                        eprintln!("Error processing packet: {}", e);
                    }
                }
                Err(e) => eprintln!("Error receiving data: {}", e),
            }
        }
        Ok(())
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    async fn process_packet(&self, data: &[u8]) -> Result<(), String> {
        let header = PacketHeader::try_from(data)?;

        let bytes = &data[PacketHeader::size()..];

        match header.packet_id {
            PacketType::Motion => {
                let _p = PacketMotionData::try_from(bytes)?;
            }
            PacketType::Session => {
                let _p = PacketSessionData::try_from(bytes)?;
                // println!("{:?}", p);
            }
            PacketType::CarTelemetry => {
                let _p = PacketCarTelemetry::try_from(bytes)?;

                // let mut session_data = self.session_data.lock().await;
                // session_data.insert(Utc::now(), telemetry.clone());

                // println!(
                //     "Speed: {} km/h, Gear: {}, Throttle: {:.2}, Brake: {:.2}",
                //     p.speed, p.gear, p.throttle, p.brake
                // );
            }
            PacketType::LapData => {
                let _p = PacketLapData::try_from(bytes)?;
            }
            _ => (),
        }

        Ok(())
    }

    pub async fn save_session_data(&self, _filename: &str) -> Result<(), io::Error> {
        // let session_data = self.session_data.lock().await;
        // let json = serde_json::to_string_pretty(&*session_data)?;
        // std::fs::write(filename, json)?;
        Ok(())
    }
}
