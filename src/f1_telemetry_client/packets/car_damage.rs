use super::Packet;

#[derive(Debug, Clone, Copy)]
pub struct CarDamageData {
    pub tyres_wear: [f32; 4],        // Tyre wear (percentage)
    pub tyres_damage: [u8; 4],       // Tyre damage (percentage)
    pub brakes_damage: [u8; 4],      // Brakes damage (percentage)
    pub front_left_wing_damage: u8,  // Front left wing damage (percentage)
    pub front_right_wing_damage: u8, // Front right wing damage (percentage)
    pub rear_wing_damage: u8,        // Rear wing damage (percentage)
    pub floor_damage: u8,            // Floor damage (percentage)
    pub diffuser_damage: u8,         // Diffuser damage (percentage)
    pub sidepod_damage: u8,          // Sidepod damage (percentage)
    pub drs_fault: u8,               // Indicator for DRS fault, 0 = OK, 1 = fault
    pub ers_fault: u8,               // Indicator for ERS fault, 0 = OK, 1 = fault
    pub gear_box_damage: u8,         // Gear box damage (percentage)
    pub engine_damage: u8,           // Engine damage (percentage)
    pub engine_mguh_wear: u8,        // Engine wear MGU-H (percentage)
    pub engine_es_wear: u8,          // Engine wear ES (percentage)
    pub engine_ce_wear: u8,          // Engine wear CE (percentage)
    pub engine_ice_wear: u8,         // Engine wear ICE (percentage)
    pub engine_mguk_wear: u8,        // Engine wear MGU-K (percentage)
    pub engine_tc_wear: u8,          // Engine wear TC (percentage)
    pub engine_blown: u8,            // Engine blown, 0 = OK, 1 = fault
    pub engine_seized: u8,           // Engine seized, 0 = OK, 1 = fault
}

#[derive(Debug, Clone)]
pub struct PacketCarDamageData {
    pub car_damage_data: Vec<CarDamageData>, // Car damage data for all cars
}

impl Packet for PacketCarDamageData {
    fn size() -> usize {
        953 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketCarDamageData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketCarDamageData::size() {
            return Err("Packet too short for PacketCarDamageData".into());
        }

        let mut car_damage_data = Vec::with_capacity(22);
        let mut offset = 0;

        // Parse damage data for each car
        for _ in 0..22 {
            // Parse tyre wear (4 f32 values)
            let mut tyres_wear = [0.0f32; 4];
            for i in 0..4 {
                tyres_wear[i] = f32::from_le_bytes([
                    bytes[offset + i * 4],
                    bytes[offset + i * 4 + 1],
                    bytes[offset + i * 4 + 2],
                    bytes[offset + i * 4 + 3],
                ]);
            }
            offset += 16;

            // Parse tyre damage (4 u8 values)
            let tyres_damage = [
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ];
            offset += 4;

            // Parse brakes damage (4 u8 values)
            let brakes_damage = [
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ];
            offset += 4;

            let damage = CarDamageData {
                tyres_wear,
                tyres_damage,
                brakes_damage,
                front_left_wing_damage: bytes[offset],
                front_right_wing_damage: bytes[offset + 1],
                rear_wing_damage: bytes[offset + 2],
                floor_damage: bytes[offset + 3],
                diffuser_damage: bytes[offset + 4],
                sidepod_damage: bytes[offset + 5],
                drs_fault: bytes[offset + 6],
                ers_fault: bytes[offset + 7],
                gear_box_damage: bytes[offset + 8],
                engine_damage: bytes[offset + 9],
                engine_mguh_wear: bytes[offset + 10],
                engine_es_wear: bytes[offset + 11],
                engine_ce_wear: bytes[offset + 12],
                engine_ice_wear: bytes[offset + 13],
                engine_mguk_wear: bytes[offset + 14],
                engine_tc_wear: bytes[offset + 15],
                engine_blown: bytes[offset + 16],
                engine_seized: bytes[offset + 17],
            };
            car_damage_data.push(damage);
            offset += 18; // Remaining size of each car damage data block
        }

        Ok(PacketCarDamageData { car_damage_data })
    }
}

impl Default for CarDamageData {
    fn default() -> Self {
        CarDamageData {
            tyres_wear: [0.0; 4],
            tyres_damage: [0; 4],
            brakes_damage: [0; 4],
            front_left_wing_damage: 0,
            front_right_wing_damage: 0,
            rear_wing_damage: 0,
            floor_damage: 0,
            diffuser_damage: 0,
            sidepod_damage: 0,
            drs_fault: 0,
            ers_fault: 0,
            gear_box_damage: 0,
            engine_damage: 0,
            engine_mguh_wear: 0,
            engine_es_wear: 0,
            engine_ce_wear: 0,
            engine_ice_wear: 0,
            engine_mguk_wear: 0,
            engine_tc_wear: 0,
            engine_blown: 0,
            engine_seized: 0,
        }
    }
}

impl Default for PacketCarDamageData {
    fn default() -> Self {
        PacketCarDamageData {
            car_damage_data: Vec::new(),
        }
    }
}
