use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,           // Lap time in milliseconds
    pub sector1_time_ms_part: u16,     // Sector 1 milliseconds part
    pub sector1_time_minutes_part: u8, // Sector 1 whole minute part
    pub sector2_time_ms_part: u16,     // Sector 2 time milliseconds part
    pub sector2_time_minutes_part: u8, // Sector 2 whole minute part
    pub sector3_time_ms_part: u16,     // Sector 3 time milliseconds part
    pub sector3_time_minutes_part: u8, // Sector 3 whole minute part
    pub lap_valid_bit_flags: u8,       // 0x01 bit set-lap valid, 0x02 bit set-sector 1 valid
                                       // 0x04 bit set-sector 2 valid, 0x08 bit set-sector 3 valid
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,              // Lap the tyre usage ends on (255 of current tyre)
    pub tyre_actual_compound: u8, // Actual tyres used by this driver
    pub tyre_visual_compound: u8, // Visual tyres used by this driver
}

#[derive(Debug, Clone, Default)]
pub struct PacketSessionHistoryData {
    pub car_idx: u8,               // Index of the car this lap data relates to
    pub num_laps: u8,              // Num laps in the data (including current partial lap)
    pub num_tyre_stints: u8,       // Number of tyre stints in the data
    pub best_lap_time_lap_num: u8, // Lap the best lap time was achieved on
    pub best_sector1_lap_num: u8,  // Lap the best Sector 1 time was achieved on
    pub best_sector2_lap_num: u8,  // Lap the best Sector 2 time was achieved on
    pub best_sector3_lap_num: u8,  // Lap the best Sector 3 time was achieved on
    pub lap_history_data: Vec<LapHistoryData>, // 100 laps of data max
    pub tyre_stints_history_data: Vec<TyreStintHistoryData>, // 8 tyre stints max
}

impl Packet for PacketSessionHistoryData {
    fn size() -> usize {
        1460 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketSessionHistoryData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketSessionHistoryData::size() {
            return Err("Packet too short for PacketSessionHistoryData".into());
        }

        let mut offset = 0;
        let car_idx = bytes[offset];
        offset += 1;
        let num_laps = bytes[offset];
        offset += 1;
        let num_tyre_stints = bytes[offset];
        offset += 1;
        let best_lap_time_lap_num = bytes[offset];
        offset += 1;
        let best_sector1_lap_num = bytes[offset];
        offset += 1;
        let best_sector2_lap_num = bytes[offset];
        offset += 1;
        let best_sector3_lap_num = bytes[offset];
        offset += 1;

        // Parse lap history data (100 laps maximum)
        let mut lap_history_data = Vec::with_capacity(100);
        for _ in 0..100 {
            let lap_data = LapHistoryData {
                lap_time_in_ms: u32::from_le_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                ]),
                sector1_time_ms_part: u16::from_le_bytes([bytes[offset + 4], bytes[offset + 5]]),
                sector1_time_minutes_part: bytes[offset + 6],
                sector2_time_ms_part: u16::from_le_bytes([bytes[offset + 7], bytes[offset + 8]]),
                sector2_time_minutes_part: bytes[offset + 9],
                sector3_time_ms_part: u16::from_le_bytes([bytes[offset + 10], bytes[offset + 11]]),
                sector3_time_minutes_part: bytes[offset + 12],
                lap_valid_bit_flags: bytes[offset + 13],
            };
            lap_history_data.push(lap_data);
            offset += 14; // Size of each lap history data block
        }

        // Parse tyre stint history data (8 stints maximum)
        let mut tyre_stints_history_data = Vec::with_capacity(8);
        for _ in 0..8 {
            let stint_data = TyreStintHistoryData {
                end_lap: bytes[offset],
                tyre_actual_compound: bytes[offset + 1],
                tyre_visual_compound: bytes[offset + 2],
            };
            tyre_stints_history_data.push(stint_data);
            offset += 3; // Size of each tyre stint data block
        }

        Ok(PacketSessionHistoryData {
            car_idx,
            num_laps,
            num_tyre_stints,
            best_lap_time_lap_num,
            best_sector1_lap_num,
            best_sector2_lap_num,
            best_sector3_lap_num,
            lap_history_data,
            tyre_stints_history_data,
        })
    }
}
