mod packets;
use chrono::{DateTime, Utc};
use packets::session_data::PacketSessionData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

use packets::PacketType;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CarTelemetry {
    speed: u16,                  // Speed of car in km/h
    throttle: f32,               // Amount of throttle applied (0.0 to 1.0)
    steer: f32,                  // Steering (-1.0 for full left to 1.0 for full right)
    brake: f32,                  // Amount of brake applied (0.0 to 1.0)
    clutch: u8,                  // Amount of clutch applied (0 to 100)
    gear: i8,                    // Gear selected (1-8, 0 = neutral, -1 = reverse)
    engine_rpm: u16,             // Engine RPM
    drs: u8,                     // 0 = off, 1 = on
    rev_lights_percent: u8,      // Rev lights indicator (percentage)
    brake_temp: [f32; 4],        // Brake temperatures (FL, FR, RL, RR)
    tyre_surface_temp: [f32; 4], // Tyre surface temperatures
    tyre_inner_temp: [f32; 4],   // Tyre inner temperatures
}

#[derive(Debug)]
pub struct PacketHeader {
    packet_format: u16,             // 2024
    game_year: u8,                  // Game year - last two digits e.g. 24
    game_major_version: u8,         // Game major version - "X.00"
    game_minor_version: u8,         // Game minor version - "1.XX"
    packet_version: u8,             // Version of this packet type
    packet_id: PacketType,          // Identifier for the packet type
    session_uid: u64,               // Unique identifier for the session
    session_time: f32,              // Session timestamp
    frame_identifier: u32,          // Frame identifier
    overall_frame_identifier: u32,  // Overall frame identifier
    player_car_index: u8,           // Index of player's car
    secondary_player_car_index: u8, // Index of secondary player's car (255 if none)
}

const HEADER_PACKET_SIZE: usize = 29;

type SessionData = HashMap<DateTime<Utc>, CarTelemetry>;

pub struct F1TelemetryClient {
    socket: Arc<UdpSocket>,
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

    async fn process_packet(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let header = self.parse_header(data)?;

        match header.packet_id {
            PacketType::Session => {
                let p = PacketSessionData::try_from(&data[HEADER_PACKET_SIZE..])?;
                println!("{:?}", p);
                ()
            }
            PacketType::CarTelemetry => {
                self.process_car_telemetry(&data[HEADER_PACKET_SIZE..])
                    .await?
            }
            _ => (),
        }

        Ok(())
    }

    fn parse_header(&self, data: &[u8]) -> Result<PacketHeader, Box<dyn Error>> {
        if data.len() < HEADER_PACKET_SIZE {
            return Err("Packet too short for header".into());
        }

        let packet_id = data[6].try_into()?;

        Ok(PacketHeader {
            packet_format: u16::from_le_bytes([data[0], data[1]]),
            game_year: data[2],
            game_major_version: data[3],
            game_minor_version: data[4],
            packet_version: data[5],
            packet_id,
            session_uid: u64::from_le_bytes([
                data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14],
            ]),
            session_time: f32::from_le_bytes([data[15], data[16], data[17], data[18]]),
            frame_identifier: u32::from_le_bytes([data[19], data[20], data[21], data[22]]),
            overall_frame_identifier: u32::from_le_bytes([data[23], data[24], data[25], data[26]]),
            player_car_index: data[27],
            secondary_player_car_index: data[28],
        })
    }

    async fn process_car_telemetry(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        if data.len() < 60 {
            // Minimum size for car telemetry data
            return Err("Packet too short for car telemetry".into());
        }

        let telemetry = CarTelemetry {
            speed: u16::from_le_bytes([data[0], data[1]]),
            throttle: f32::from_le_bytes([data[2], data[3], data[4], data[5]]),
            steer: f32::from_le_bytes([data[6], data[7], data[8], data[9]]),
            brake: f32::from_le_bytes([data[10], data[11], data[12], data[13]]),
            clutch: data[14],
            gear: data[15] as i8,
            engine_rpm: u16::from_le_bytes([data[16], data[17]]),
            drs: data[18],
            rev_lights_percent: data[19],
            brake_temp: [
                f32::from_le_bytes([data[20], data[21], data[22], data[23]]),
                f32::from_le_bytes([data[24], data[25], data[26], data[27]]),
                f32::from_le_bytes([data[28], data[29], data[30], data[31]]),
                f32::from_le_bytes([data[32], data[33], data[34], data[35]]),
            ],
            tyre_surface_temp: [
                f32::from_le_bytes([data[36], data[37], data[38], data[39]]),
                f32::from_le_bytes([data[40], data[41], data[42], data[43]]),
                f32::from_le_bytes([data[44], data[45], data[46], data[47]]),
                f32::from_le_bytes([data[48], data[49], data[50], data[51]]),
            ],
            tyre_inner_temp: [
                f32::from_le_bytes([data[52], data[53], data[54], data[55]]),
                f32::from_le_bytes([data[56], data[57], data[58], data[59]]),
                f32::from_le_bytes([data[60], data[61], data[62], data[63]]),
                f32::from_le_bytes([data[64], data[65], data[66], data[67]]),
            ],
        };

        // Store the telemetry data
        let mut session_data = self.session_data.lock().await;
        session_data.insert(Utc::now(), telemetry.clone());

        // Print real-time data
        println!(
            "Speed: {} km/h, Gear: {}, Throttle: {:.2}, Brake: {:.2}",
            telemetry.speed, telemetry.gear, telemetry.throttle, telemetry.brake
        );

        Ok(())
    }

    pub async fn save_session_data(&self, filename: &str) -> Result<(), io::Error> {
        let session_data = self.session_data.lock().await;
        let json = serde_json::to_string_pretty(&*session_data)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
}
