use super::Packet;

#[derive(Debug, Clone, Copy)]
pub struct FinalClassificationData {
    pub position: u8,      // Finishing position
    pub num_laps: u8,      // Number of laps completed
    pub grid_position: u8, // Grid position of the car
    pub points: u8,        // Number of points scored
    pub num_pit_stops: u8, // Number of pit stops made
    pub result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub best_lap_time_in_ms: u32, // Best lap time of the session in milliseconds
    pub total_race_time: f64,     // Total race time in seconds without penalties
    pub penalties_time: u8,       // Total penalties accumulated in seconds
    pub num_penalties: u8,        // Number of penalties applied to this driver
    pub num_tyre_stints: u8,      // Number of tyres stints up to maximum
    pub tyre_stints_actual: [u8; 8], // Actual tyres used by this driver
    pub tyre_stints_visual: [u8; 8], // Visual tyres used by this driver
    pub tyre_stints_end_laps: [u8; 8], // The lap number stints end on
}

#[derive(Debug, Clone)]
pub struct PacketFinalClassificationData {
    pub num_cars: u8, // Number of cars in the final classification
    pub classification_data: Vec<FinalClassificationData>, // Final classification data for all cars
}

impl Packet for PacketFinalClassificationData {
    fn size() -> usize {
        1020 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketFinalClassificationData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketFinalClassificationData::size() {
            return Err("Packet too short for PacketFinalClassificationData".into());
        }

        let num_cars = bytes[0];
        let mut classification_data = Vec::with_capacity(22);
        let mut offset = 1;

        // Parse classification data for each car
        for _ in 0..22 {
            let data = FinalClassificationData {
                position: bytes[offset],
                num_laps: bytes[offset + 1],
                grid_position: bytes[offset + 2],
                points: bytes[offset + 3],
                num_pit_stops: bytes[offset + 4],
                result_status: bytes[offset + 5],
                best_lap_time_in_ms: u32::from_le_bytes([
                    bytes[offset + 6],
                    bytes[offset + 7],
                    bytes[offset + 8],
                    bytes[offset + 9],
                ]),
                total_race_time: f64::from_le_bytes([
                    bytes[offset + 10],
                    bytes[offset + 11],
                    bytes[offset + 12],
                    bytes[offset + 13],
                    bytes[offset + 14],
                    bytes[offset + 15],
                    bytes[offset + 16],
                    bytes[offset + 17],
                ]),
                penalties_time: bytes[offset + 18],
                num_penalties: bytes[offset + 19],
                num_tyre_stints: bytes[offset + 20],
                tyre_stints_actual: [
                    bytes[offset + 21],
                    bytes[offset + 22],
                    bytes[offset + 23],
                    bytes[offset + 24],
                    bytes[offset + 25],
                    bytes[offset + 26],
                    bytes[offset + 27],
                    bytes[offset + 28],
                ],
                tyre_stints_visual: [
                    bytes[offset + 29],
                    bytes[offset + 30],
                    bytes[offset + 31],
                    bytes[offset + 32],
                    bytes[offset + 33],
                    bytes[offset + 34],
                    bytes[offset + 35],
                    bytes[offset + 36],
                ],
                tyre_stints_end_laps: [
                    bytes[offset + 37],
                    bytes[offset + 38],
                    bytes[offset + 39],
                    bytes[offset + 40],
                    bytes[offset + 41],
                    bytes[offset + 42],
                    bytes[offset + 43],
                    bytes[offset + 44],
                ],
            };
            classification_data.push(data);
            offset += 45; // Size of each classification data block
        }

        Ok(PacketFinalClassificationData {
            num_cars,
            classification_data,
        })
    }
}

impl Default for FinalClassificationData {
    fn default() -> Self {
        FinalClassificationData {
            position: 0,
            num_laps: 0,
            grid_position: 0,
            points: 0,
            num_pit_stops: 0,
            result_status: 0,
            best_lap_time_in_ms: 0,
            total_race_time: 0.0,
            penalties_time: 0,
            num_penalties: 0,
            num_tyre_stints: 0,
            tyre_stints_actual: [0; 8],
            tyre_stints_visual: [0; 8],
            tyre_stints_end_laps: [0; 8],
        }
    }
}

impl Default for PacketFinalClassificationData {
    fn default() -> Self {
        PacketFinalClassificationData {
            num_cars: 0,
            classification_data: Vec::new(),
        }
    }
}
