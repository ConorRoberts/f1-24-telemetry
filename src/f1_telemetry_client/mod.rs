mod packets;

use packets::car_motion_data::PacketMotionData;
use packets::car_telemetry::PacketCarTelemetry;
use packets::header::PacketHeader;
use packets::lap_data::PacketLapData;
use packets::session_data::PacketSessionData;
use packets::{header::PacketType, PacketSize};
use std::error::Error;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{debug, error};

pub struct F1TelemetryClient {
    socket: Arc<UdpSocket>,
    running: Arc<Mutex<bool>>,
}

pub enum TelemetryPacket {
    Session((PacketHeader, PacketSessionData)),
    Motion((PacketHeader, PacketMotionData)),
    CarTelemetry((PacketHeader, PacketCarTelemetry)),
    LapData((PacketHeader, PacketLapData)),
}

impl TryFrom<&[u8]> for TelemetryPacket {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<TelemetryPacket, Self::Error> {
        let header = PacketHeader::try_from(value)?;

        debug!(
            "Received packet \"{:?}\" of size {}",
            header.packet_id,
            value.len() - PacketHeader::size()
        );

        let bytes = &value[PacketHeader::size()..];

        match header.packet_id {
            PacketType::Motion => Ok(Self::Motion((header, PacketMotionData::try_from(bytes)?))),
            PacketType::Session => Ok(Self::Session((header, PacketSessionData::try_from(bytes)?))),
            PacketType::CarTelemetry => Ok(Self::CarTelemetry((
                header,
                PacketCarTelemetry::try_from(bytes)?,
            ))),
            PacketType::LapData => Ok(Self::LapData((header, PacketLapData::try_from(bytes)?))),
            _ => Err(format!("Unsupported packet type {:?}", header.packet_id)),
        }
    }
}

impl F1TelemetryClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket: Arc::new(socket),
            running: Arc::new(Mutex::new(true)),
        })
    }

    pub async fn start<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(TelemetryPacket) -> (),
    {
        println!("Listening for F1 24 telemetry data...");
        let mut buf = [0u8; 2048];

        while *self.running.lock().await {
            match self.socket.recv(&mut buf).await {
                Ok(size) => match TelemetryPacket::try_from(&buf[..size]) {
                    Ok(p) => f(p),
                    Err(e) => {
                        error!("Error processing packet: {}", e);
                    }
                },
                Err(e) => error!("Error receiving data: {}", e),
            }
        }
        Ok(())
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }
}
