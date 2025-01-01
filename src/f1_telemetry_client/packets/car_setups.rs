use super::PacketSize;

#[derive(Debug, Clone, Copy)]
pub struct CarSetupData {
    pub front_wing: u8,                 // Front wing aero
    pub rear_wing: u8,                  // Rear wing aero
    pub on_throttle: u8,                // Differential adjustment on throttle (percentage)
    pub off_throttle: u8,               // Differential adjustment off throttle (percentage)
    pub front_camber: f32,              // Front camber angle (suspension geometry)
    pub rear_camber: f32,               // Rear camber angle (suspension geometry)
    pub front_toe: f32,                 // Front toe angle (suspension geometry)
    pub rear_toe: f32,                  // Rear toe angle (suspension geometry)
    pub front_suspension: u8,           // Front suspension
    pub rear_suspension: u8,            // Rear suspension
    pub front_anti_roll_bar: u8,        // Front anti-roll bar
    pub rear_anti_roll_bar: u8,         // Front anti-roll bar
    pub front_suspension_height: u8,    // Front ride height
    pub rear_suspension_height: u8,     // Rear ride height
    pub brake_pressure: u8,             // Brake pressure (percentage)
    pub brake_bias: u8,                 // Brake bias (percentage)
    pub engine_braking: u8,             // Engine braking (percentage)
    pub rear_left_tyre_pressure: f32,   // Rear left tyre pressure (PSI)
    pub rear_right_tyre_pressure: f32,  // Rear right tyre pressure (PSI)
    pub front_left_tyre_pressure: f32,  // Front left tyre pressure (PSI)
    pub front_right_tyre_pressure: f32, // Front right tyre pressure (PSI)
    pub ballast: u8,                    // Ballast
    pub fuel_load: f32,                 // Fuel load
}

#[derive(Debug, Clone)]
pub struct PacketCarSetupData {
    pub car_setups: Vec<CarSetupData>, // Car setups for all cars
    pub next_front_wing_value: f32,    // Value of front wing after next pit stop - player only
}

impl PacketSize for PacketCarSetupData {
    fn size() -> usize {
        1133 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketCarSetupData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketCarSetupData::size() {
            return Err("Packet too short for PacketCarSetupData".into());
        }

        let mut car_setups = Vec::with_capacity(22);
        let mut offset = 0;

        // Parse setup data for each car
        for _ in 0..22 {
            let setup = CarSetupData {
                front_wing: bytes[offset],
                rear_wing: bytes[offset + 1],
                on_throttle: bytes[offset + 2],
                off_throttle: bytes[offset + 3],
                front_camber: f32::from_le_bytes([
                    bytes[offset + 4],
                    bytes[offset + 5],
                    bytes[offset + 6],
                    bytes[offset + 7],
                ]),
                rear_camber: f32::from_le_bytes([
                    bytes[offset + 8],
                    bytes[offset + 9],
                    bytes[offset + 10],
                    bytes[offset + 11],
                ]),
                front_toe: f32::from_le_bytes([
                    bytes[offset + 12],
                    bytes[offset + 13],
                    bytes[offset + 14],
                    bytes[offset + 15],
                ]),
                rear_toe: f32::from_le_bytes([
                    bytes[offset + 16],
                    bytes[offset + 17],
                    bytes[offset + 18],
                    bytes[offset + 19],
                ]),
                front_suspension: bytes[offset + 20],
                rear_suspension: bytes[offset + 21],
                front_anti_roll_bar: bytes[offset + 22],
                rear_anti_roll_bar: bytes[offset + 23],
                front_suspension_height: bytes[offset + 24],
                rear_suspension_height: bytes[offset + 25],
                brake_pressure: bytes[offset + 26],
                brake_bias: bytes[offset + 27],
                engine_braking: bytes[offset + 28],
                rear_left_tyre_pressure: f32::from_le_bytes([
                    bytes[offset + 29],
                    bytes[offset + 30],
                    bytes[offset + 31],
                    bytes[offset + 32],
                ]),
                rear_right_tyre_pressure: f32::from_le_bytes([
                    bytes[offset + 33],
                    bytes[offset + 34],
                    bytes[offset + 35],
                    bytes[offset + 36],
                ]),
                front_left_tyre_pressure: f32::from_le_bytes([
                    bytes[offset + 37],
                    bytes[offset + 38],
                    bytes[offset + 39],
                    bytes[offset + 40],
                ]),
                front_right_tyre_pressure: f32::from_le_bytes([
                    bytes[offset + 41],
                    bytes[offset + 42],
                    bytes[offset + 43],
                    bytes[offset + 44],
                ]),
                ballast: bytes[offset + 45],
                fuel_load: f32::from_le_bytes([
                    bytes[offset + 46],
                    bytes[offset + 47],
                    bytes[offset + 48],
                    bytes[offset + 49],
                ]),
            };
            car_setups.push(setup);
            offset += 50; // Size of each car setup data block
        }

        // Parse next front wing value (after all car setups)
        let next_front_wing_value = f32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);

        Ok(PacketCarSetupData {
            car_setups,
            next_front_wing_value,
        })
    }
}

impl Default for CarSetupData {
    fn default() -> Self {
        CarSetupData {
            front_wing: 0,
            rear_wing: 0,
            on_throttle: 0,
            off_throttle: 0,
            front_camber: 0.0,
            rear_camber: 0.0,
            front_toe: 0.0,
            rear_toe: 0.0,
            front_suspension: 0,
            rear_suspension: 0,
            front_anti_roll_bar: 0,
            rear_anti_roll_bar: 0,
            front_suspension_height: 0,
            rear_suspension_height: 0,
            brake_pressure: 0,
            brake_bias: 0,
            engine_braking: 0,
            rear_left_tyre_pressure: 0.0,
            rear_right_tyre_pressure: 0.0,
            front_left_tyre_pressure: 0.0,
            front_right_tyre_pressure: 0.0,
            ballast: 0,
            fuel_load: 0.0,
        }
    }
}

impl Default for PacketCarSetupData {
    fn default() -> Self {
        PacketCarSetupData {
            car_setups: Vec::new(),
            next_front_wing_value: 0.0,
        }
    }
}
