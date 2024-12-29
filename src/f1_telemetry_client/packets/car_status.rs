use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct CarStatusData {
    pub traction_control: u8,         // 0 = off, 1 = medium, 2 = full
    pub anti_lock_brakes: u8,         // 0 (off) - 1 (on)
    pub fuel_mix: u8,                 // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub front_brake_bias: u8,         // Front brake bias (percentage)
    pub pit_limiter_status: u8,       // Pit limiter status - 0 = off, 1 = on
    pub fuel_in_tank: f32,            // Current fuel mass
    pub fuel_capacity: f32,           // Fuel capacity
    pub fuel_remaining_laps: f32,     // Fuel remaining in terms of laps (value on MFD)
    pub max_rpm: u16,                 // Cars max RPM, point of rev limiter
    pub idle_rpm: u16,                // Cars idle RPM
    pub max_gears: u8,                // Maximum number of gears
    pub drs_allowed: u8,              // 0 = not allowed, 1 = allowed
    pub drs_activation_distance: u16, // 0 = DRS not available, non-zero - DRS will be available in [X] metres
    pub actual_tyre_compound: u8,     // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 21 = C0, 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 -- 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub visual_tyre_compound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic -- same as above
    // F2 '19, 15 = wet, 19 -- super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub tyres_age_laps: u8,    // Age in laps of the current set of tyres
    pub vehicle_fia_flags: i8, // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow
    pub engine_power_ice: f32,  // Engine power output of ICE (W)
    pub engine_power_mguk: f32, // Engine power output of MGU-K (W)
    pub ers_store_energy: f32,  // ERS energy store in Joules
    pub ers_deploy_mode: u8,    // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    pub ers_harvested_this_lap_mguk: f32, // ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mguh: f32, // ERS energy harvested this lap by MGU-H
    pub ers_deployed_this_lap: f32,       // ERS energy deployed this lap
    pub network_paused: bool,             // Whether the car is paused in a network game
}

#[derive(Debug, Clone, Default)]
pub struct PacketCarStatusData {
    pub car_status_data: Vec<CarStatusData>, // Car status for all 22 cars
}

impl Packet for PacketCarStatusData {
    fn size() -> usize {
        1239 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketCarStatusData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketCarStatusData::size() {
            return Err("Packet too short for PacketCarStatusData".into());
        }

        let mut car_status_data = Vec::with_capacity(22);
        let mut offset = 0;

        // Parse status data for each car
        for _ in 0..22 {
            let status = CarStatusData {
                traction_control: bytes[offset],
                anti_lock_brakes: bytes[offset + 1],
                fuel_mix: bytes[offset + 2],
                front_brake_bias: bytes[offset + 3],
                pit_limiter_status: bytes[offset + 4],
                fuel_in_tank: f32::from_le_bytes([
                    bytes[offset + 5],
                    bytes[offset + 6],
                    bytes[offset + 7],
                    bytes[offset + 8],
                ]),
                fuel_capacity: f32::from_le_bytes([
                    bytes[offset + 9],
                    bytes[offset + 10],
                    bytes[offset + 11],
                    bytes[offset + 12],
                ]),
                fuel_remaining_laps: f32::from_le_bytes([
                    bytes[offset + 13],
                    bytes[offset + 14],
                    bytes[offset + 15],
                    bytes[offset + 16],
                ]),
                max_rpm: u16::from_le_bytes([bytes[offset + 17], bytes[offset + 18]]),
                idle_rpm: u16::from_le_bytes([bytes[offset + 19], bytes[offset + 20]]),
                max_gears: bytes[offset + 21],
                drs_allowed: bytes[offset + 22],
                drs_activation_distance: u16::from_le_bytes([
                    bytes[offset + 23],
                    bytes[offset + 24],
                ]),
                actual_tyre_compound: bytes[offset + 25],
                visual_tyre_compound: bytes[offset + 26],
                tyres_age_laps: bytes[offset + 27],
                vehicle_fia_flags: bytes[offset + 28] as i8,
                engine_power_ice: f32::from_le_bytes([
                    bytes[offset + 29],
                    bytes[offset + 30],
                    bytes[offset + 31],
                    bytes[offset + 32],
                ]),
                engine_power_mguk: f32::from_le_bytes([
                    bytes[offset + 33],
                    bytes[offset + 34],
                    bytes[offset + 35],
                    bytes[offset + 36],
                ]),
                ers_store_energy: f32::from_le_bytes([
                    bytes[offset + 37],
                    bytes[offset + 38],
                    bytes[offset + 39],
                    bytes[offset + 40],
                ]),
                ers_deploy_mode: bytes[offset + 41],
                ers_harvested_this_lap_mguk: f32::from_le_bytes([
                    bytes[offset + 42],
                    bytes[offset + 43],
                    bytes[offset + 44],
                    bytes[offset + 45],
                ]),
                ers_harvested_this_lap_mguh: f32::from_le_bytes([
                    bytes[offset + 46],
                    bytes[offset + 47],
                    bytes[offset + 48],
                    bytes[offset + 49],
                ]),
                ers_deployed_this_lap: f32::from_le_bytes([
                    bytes[offset + 50],
                    bytes[offset + 51],
                    bytes[offset + 52],
                    bytes[offset + 53],
                ]),
                network_paused: bytes[offset + 54] != 0,
            };
            car_status_data.push(status);
            offset += 55; // Size of each car status data block
        }

        Ok(PacketCarStatusData { car_status_data })
    }
}
