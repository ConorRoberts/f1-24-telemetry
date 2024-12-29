use super::{Packet, PacketType};

#[derive(Debug)]
pub struct PacketHeader {
    pub packet_format: u16,             // 2024
    pub game_year: u8,                  // Game year - last two digits e.g. 24
    pub game_major_version: u8,         // Game major version - "X.00"
    pub game_minor_version: u8,         // Game minor version - "1.XX"
    pub packet_version: u8,             // Version of this packet type
    pub packet_id: PacketType,          // Identifier for the packet type
    pub session_uid: u64,               // Unique identifier for the session
    pub session_time: f32,              // Session timestamp
    pub frame_identifier: u32,          // Frame identifier
    pub overall_frame_identifier: u32,  // Overall frame identifier
    pub player_car_index: u8,           // Index of player's car
    pub secondary_player_car_index: u8, // Index of secondary player's car (255 if none)
}

impl Packet for PacketHeader {
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
}
