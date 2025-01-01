use super::PacketSize;

#[derive(Debug, Clone, Copy, Default)]
pub struct LapData {
    // Last lap time in milliseconds
    pub last_lap_time_in_ms: u32,
    // Current time around the lap in milliseconds
    pub current_lap_time_in_ms: u32,
    // Sector 1 time milliseconds part
    pub sector1_time_ms_part: u16,
    // Sector 1 whole minute part
    pub sector1_time_minutes_part: u8,
    // Sector 2 time milliseconds part
    pub sector2_time_ms_part: u16,
    // Sector 2 whole minute part
    pub sector2_time_minutes_part: u8,
    // Time delta to car in front milliseconds part
    pub delta_to_car_in_front_ms_part: u16,
    // Time delta to car in front whole minute part
    pub delta_to_car_in_front_minutes_part: u8,
    // Time delta to race leader milliseconds part
    pub delta_to_race_leader_ms_part: u16,
    // Time delta to race leader whole minute part
    pub delta_to_race_leader_minutes_part: u8,
    // Distance vehicle is around current lap in metres
    pub lap_distance: f32,
    // Total distance travelled in session in metres
    pub total_distance: f32,
    // Delta in seconds for safety car
    pub safety_car_delta: f32,
    // Car race position
    pub car_position: u8,
    // Current lap number
    pub current_lap_num: u8,
    // 0 = none, 1 = pitting, 2 = in pit area
    pub pit_status: u8,
    // Number of pit stops taken in this race
    pub num_pit_stops: u8,
    // 0 = sector1, 1 = sector2, 2 = sector3
    pub sector: u8,
    // Current lap invalid - 0 = valid, 1 = invalid
    pub current_lap_invalid: u8,
    // Accumulated time penalties in seconds to be added
    pub penalties: u8,
    // Accumulated number of warnings issued
    pub total_warnings: u8,
    // Accumulated number of corner cutting warnings issued
    pub corner_cutting_warnings: u8,
    // Num drive through pens left to serve
    pub num_unserved_drive_through_pens: u8,
    // Num stop go pens left to serve
    pub num_unserved_stop_go_pens: u8,
    // Grid position the vehicle started the race in
    pub grid_position: u8,
    // Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    pub driver_status: u8,
    // Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = dnf, 5 = dsq, 6 = not classified, 7 = retired
    pub result_status: u8,
    // Pit lane timing, 0 = inactive, 1 = active
    pub pit_lane_timer_active: u8,
    // If active, the current time spent in the pit lane in ms
    pub pit_lane_time_in_lane_in_ms: u16,
    // Time of the actual pit stop in ms
    pub pit_stop_timer_in_ms: u16,
    // Whether the car should serve a penalty at this stop
    pub pit_stop_should_serve_pen: u8,
    // Fastest speed through speed trap for this car in kmph
    pub speed_trap_fastest_speed: f32,
    // Lap no the fastest speed was achieved, 255 = not set
    pub speed_trap_fastest_lap: u8,
}

impl PacketSize for LapData {
    fn size() -> usize {
        57
    }
}

impl TryFrom<&[u8]> for LapData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < LapData::size() {
            return Err("Buffer too small for LapData".into());
        }

        Ok(Self {
            last_lap_time_in_ms: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            current_lap_time_in_ms: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            sector1_time_ms_part: u16::from_le_bytes([bytes[8], bytes[9]]),
            sector1_time_minutes_part: bytes[10],
            sector2_time_ms_part: u16::from_le_bytes([bytes[11], bytes[12]]),
            sector2_time_minutes_part: bytes[13],
            delta_to_car_in_front_ms_part: u16::from_le_bytes([bytes[14], bytes[15]]),
            delta_to_car_in_front_minutes_part: bytes[16],
            delta_to_race_leader_ms_part: u16::from_le_bytes([bytes[17], bytes[18]]),
            delta_to_race_leader_minutes_part: bytes[19],
            lap_distance: f32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            total_distance: f32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            safety_car_delta: f32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
            car_position: bytes[32],
            current_lap_num: bytes[33],
            pit_status: bytes[34],
            num_pit_stops: bytes[35],
            sector: bytes[36],
            current_lap_invalid: bytes[37],
            penalties: bytes[38],
            total_warnings: bytes[39],
            corner_cutting_warnings: bytes[40],
            num_unserved_drive_through_pens: bytes[41],
            num_unserved_stop_go_pens: bytes[42],
            grid_position: bytes[43],
            driver_status: bytes[44],
            result_status: bytes[45],
            pit_lane_timer_active: bytes[46],
            pit_lane_time_in_lane_in_ms: u16::from_le_bytes([bytes[47], bytes[48]]),
            pit_stop_timer_in_ms: u16::from_le_bytes([bytes[49], bytes[50]]),
            pit_stop_should_serve_pen: bytes[51],
            speed_trap_fastest_speed: f32::from_le_bytes([
                bytes[52], bytes[53], bytes[54], bytes[55],
            ]),
            speed_trap_fastest_lap: bytes[56],
        })
    }
}

#[derive(Debug, Clone)]
pub struct PacketLapData {
    // Lap data for all cars on track
    pub lap_data: [LapData; 22],
    // Index of Personal Best car in time trial (255 if invalid)
    pub time_trial_pb_car_idx: u8,
    // Index of Rival car in time trial (255 if invalid)
    pub time_trial_rival_car_idx: u8,
}

impl PacketSize for PacketLapData {
    fn size() -> usize {
        1256
    }
}

impl TryFrom<&[u8]> for PacketLapData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketLapData::size() {
            return Err("Buffer too small for PacketLapData".into());
        }

        let mut lap_data = [LapData::default(); 22];
        for i in 0..22 {
            let start = i * LapData::size();
            lap_data[i] = LapData::try_from(&bytes[start..start + LapData::size()])?;
        }

        Ok(Self {
            lap_data,
            time_trial_pb_car_idx: bytes[22 * LapData::size()],
            time_trial_rival_car_idx: bytes[(22 * LapData::size()) + 1],
        })
    }
}
