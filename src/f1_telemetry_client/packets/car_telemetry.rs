use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct PacketCarTelemetry {
    pub speed: u16,                  // Speed of car in km/h
    pub throttle: f32,               // Amount of throttle applied (0.0 to 1.0)
    pub steer: f32,                  // Steering (-1.0 for full left to 1.0 for full right)
    pub brake: f32,                  // Amount of brake applied (0.0 to 1.0)
    pub clutch: u8,                  // Amount of clutch applied (0 to 100)
    pub gear: i8,                    // Gear selected (1-8, 0 = neutral, -1 = reverse)
    pub engine_rpm: u16,             // Engine RPM
    pub drs: u8,                     // 0 = off, 1 = on
    pub rev_lights_percent: u8,      // Rev lights indicator (percentage)
    pub brake_temp: [f32; 4],        // Brake temperatures (FL, FR, RL, RR)
    pub tyre_surface_temp: [f32; 4], // Tyre surface temperatures
    pub tyre_inner_temp: [f32; 4],   // Tyre inner temperatures
}

impl Packet for PacketCarTelemetry {
    fn size() -> usize {
        60
    }
}

impl TryFrom<&[u8]> for PacketCarTelemetry {
    type Error = String;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketCarTelemetry::size() {
            // Minimum size for car telemetry data
            return Err("Packet too short for PacketCarTelemetry".into());
        }

        return Ok(PacketCarTelemetry {
            speed: u16::from_le_bytes([bytes[0], bytes[1]]),
            throttle: f32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
            steer: f32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]),
            brake: f32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]),
            clutch: bytes[14],
            gear: bytes[15] as i8,
            engine_rpm: u16::from_le_bytes([bytes[16], bytes[17]]),
            drs: bytes[18],
            rev_lights_percent: bytes[19],
            brake_temp: [
                f32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
                f32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
                f32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
                f32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]),
            ],
            tyre_surface_temp: [
                f32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
                f32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]),
                f32::from_le_bytes([bytes[44], bytes[45], bytes[46], bytes[47]]),
                f32::from_le_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]),
            ],
            tyre_inner_temp: [
                f32::from_le_bytes([bytes[52], bytes[53], bytes[54], bytes[55]]),
                f32::from_le_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]),
                f32::from_le_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]),
                f32::from_le_bytes([bytes[64], bytes[65], bytes[66], bytes[67]]),
            ],
        });
    }
}
