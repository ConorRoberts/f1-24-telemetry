use super::PacketSize;

#[derive(Debug)]
pub enum PacketType {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
    TyreSets,
    MotionEx,
    TimeTrial,
}

impl TryFrom<u8> for PacketType {
    type Error = String;

    fn try_from(value: u8) -> Result<PacketType, String> {
        match value {
            0 => Ok(Self::Motion),
            1 => Ok(Self::Session),
            2 => Ok(Self::LapData),
            3 => Ok(Self::Event),
            4 => Ok(Self::Participants),
            5 => Ok(Self::CarSetups),
            6 => Ok(Self::CarTelemetry),
            7 => Ok(Self::CarStatus),
            8 => Ok(Self::FinalClassification),
            9 => Ok(Self::LobbyInfo),
            10 => Ok(Self::CarDamage),
            11 => Ok(Self::SessionHistory),
            12 => Ok(Self::TyreSets),
            13 => Ok(Self::MotionEx),
            14 => Ok(Self::TimeTrial),
            _ => Err(format!("Invalid packet type: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct PacketHeader {
    pub packet_format: u16,                     // 2024
    pub game_year: u8,                          // Game year - last two digits e.g. 24
    pub game_major_version: u8,                 // Game major version - "X.00"
    pub game_minor_version: u8,                 // Game minor version - "1.XX"
    pub packet_version: u8,                     // Version of this packet type
    pub packet_id: PacketType,                  // Identifier for the packet type
    pub session_uid: u64,                       // Unique identifier for the session
    pub session_time: f32,                      // Session timestamp
    pub frame_identifier: u32,                  // Frame identifier
    pub overall_frame_identifier: u32,          // Overall frame identifier
    pub player_car_index: u8,                   // Index of player's car
    pub secondary_player_car_index: Option<u8>, // Index of secondary player's car (255 if none)
}

impl PacketSize for PacketHeader {
    fn size() -> usize {
        29
    }
}

impl TryFrom<&[u8]> for PacketHeader {
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < PacketHeader::size() {
            return Err("Packet too short for header".into());
        }

        let packet_id: PacketType = data[6].try_into()?;

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
            secondary_player_car_index: if data[28] == 255 {
                None
            } else {
                Some(data[28])
            },
        })
    }
}
