use super::PacketSize;

/// Final race result for a driver
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    DidNotFinish,
    Disqualified,
    NotClassified,
    Retired,
}

impl Default for ResultStatus {
    fn default() -> Self {
        Self::Invalid
    }
}

impl TryFrom<u8> for ResultStatus {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::Inactive),
            2 => Ok(Self::Active),
            3 => Ok(Self::Finished),
            4 => Ok(Self::DidNotFinish),
            5 => Ok(Self::Disqualified),
            6 => Ok(Self::NotClassified),
            7 => Ok(Self::Retired),
            _ => Err(format!("Invalid value: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FinalClassificationData {
    pub position: u8,                  // Finishing position
    pub num_laps: u8,                  // Number of laps completed
    pub grid_position: u8,             // Grid position of the car
    pub points: u8,                    // Number of points scored
    pub num_pit_stops: u8,             // Number of pit stops made
    pub result_status: ResultStatus,   // Result status
    pub best_lap_time_in_ms: u32,      // Best lap time of the session in milliseconds
    pub total_race_time: f64,          // Total race time in seconds without penalties
    pub penalties_time: u8,            // Total penalties accumulated in seconds
    pub num_penalties: u8,             // Number of penalties applied to this driver
    pub num_tyre_stints: u8,           // Number of tyres stints up to maximum
    pub tyre_stints_actual: [u8; 8],   // Actual tyres used by this driver
    pub tyre_stints_visual: [u8; 8],   // Visual tyres used by this driver
    pub tyre_stints_end_laps: [u8; 8], // The lap number stints end on
}

impl PacketSize for FinalClassificationData {
    fn size() -> usize {
        45
    }
}

#[derive(Debug, Clone, Default)]
pub struct PacketFinalClassificationData {
    pub num_cars: u8, // Number of cars in the final classification
    pub classification_data: Vec<FinalClassificationData>, // Final classification data for all cars
}

impl PacketSize for PacketFinalClassificationData {
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

        // Parse classification data for each car
        for i in 0..22 {
            let offset = 1 + FinalClassificationData::size() * i;
            let data = FinalClassificationData {
                position: bytes[offset],
                num_laps: bytes[offset + 1],
                grid_position: bytes[offset + 2],
                points: bytes[offset + 3],
                num_pit_stops: bytes[offset + 4],
                result_status: ResultStatus::try_from(bytes[offset + 5])?,
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
        }

        Ok(PacketFinalClassificationData {
            num_cars,
            classification_data,
        })
    }
}
