use super::Packet;

#[derive(Debug, Clone, Copy)]
pub struct TimeTrialDataSet {
    pub car_idx: u8,               // Index of the car this data relates to
    pub team_id: u8,               // Team id - see appendix
    pub lap_time_in_ms: u32,       // Lap time in milliseconds
    pub sector1_time_in_ms: u32,   // Sector 1 time in milliseconds
    pub sector2_time_in_ms: u32,   // Sector 2 time in milliseconds
    pub sector3_time_in_ms: u32,   // Sector 3 time in milliseconds
    pub traction_control: u8,      // 0 = off, 1 = medium, 2 = full
    pub gearbox_assist: u8,        // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub anti_lock_brakes: u8,      // 0 (off) - 1 (on)
    pub equal_car_performance: u8, // 0 = Realistic, 1 = Equal
    pub custom_setup: u8,          // 0 = No, 1 = Yes
    pub valid: u8,                 // 0 = invalid, 1 = valid
}

#[derive(Debug, Clone, Copy)]
pub struct PacketTimeTrialData {
    pub player_session_best_data_set: TimeTrialDataSet, // Player session best data set
    pub personal_best_data_set: TimeTrialDataSet,       // Personal best data set
    pub rival_data_set: TimeTrialDataSet,               // Rival data set
}

impl Packet for PacketTimeTrialData {
    fn size() -> usize {
        101 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketTimeTrialData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketTimeTrialData::size() {
            return Err("Packet too short for PacketTimeTrialData".into());
        }

        // Helper function to parse a single TimeTrialDataSet
        let parse_data_set = |offset: usize| -> TimeTrialDataSet {
            TimeTrialDataSet {
                car_idx: bytes[offset],
                team_id: bytes[offset + 1],
                lap_time_in_ms: u32::from_le_bytes([
                    bytes[offset + 2],
                    bytes[offset + 3],
                    bytes[offset + 4],
                    bytes[offset + 5],
                ]),
                sector1_time_in_ms: u32::from_le_bytes([
                    bytes[offset + 6],
                    bytes[offset + 7],
                    bytes[offset + 8],
                    bytes[offset + 9],
                ]),
                sector2_time_in_ms: u32::from_le_bytes([
                    bytes[offset + 10],
                    bytes[offset + 11],
                    bytes[offset + 12],
                    bytes[offset + 13],
                ]),
                sector3_time_in_ms: u32::from_le_bytes([
                    bytes[offset + 14],
                    bytes[offset + 15],
                    bytes[offset + 16],
                    bytes[offset + 17],
                ]),
                traction_control: bytes[offset + 18],
                gearbox_assist: bytes[offset + 19],
                anti_lock_brakes: bytes[offset + 20],
                equal_car_performance: bytes[offset + 21],
                custom_setup: bytes[offset + 22],
                valid: bytes[offset + 23],
            }
        };

        // Parse the three data sets at their respective offsets
        let player_session_best_data_set = parse_data_set(0);
        let personal_best_data_set = parse_data_set(33); // Size of each data set is 33 bytes
        let rival_data_set = parse_data_set(66);

        Ok(PacketTimeTrialData {
            player_session_best_data_set,
            personal_best_data_set,
            rival_data_set,
        })
    }
}

impl Default for TimeTrialDataSet {
    fn default() -> Self {
        TimeTrialDataSet {
            car_idx: 0,
            team_id: 0,
            lap_time_in_ms: 0,
            sector1_time_in_ms: 0,
            sector2_time_in_ms: 0,
            sector3_time_in_ms: 0,
            traction_control: 0,
            gearbox_assist: 1, // Default to manual
            anti_lock_brakes: 0,
            equal_car_performance: 0,
            custom_setup: 0,
            valid: 0,
        }
    }
}

impl Default for PacketTimeTrialData {
    fn default() -> Self {
        PacketTimeTrialData {
            player_session_best_data_set: TimeTrialDataSet::default(),
            personal_best_data_set: TimeTrialDataSet::default(),
            rival_data_set: TimeTrialDataSet::default(),
        }
    }
}
