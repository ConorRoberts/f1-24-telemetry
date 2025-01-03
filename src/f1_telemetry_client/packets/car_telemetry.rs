use tracing::info;

use super::PacketSize;

#[derive(Debug, Clone, Copy, Default)]
pub struct PacketCarTelemetry {
    pub speed: u16,                 // Speed of car in km/h
    pub throttle: f32,              // Amount of throttle applied (0.0 to 1.0)
    pub steer: f32,                 // Steering (-1.0 for full left to 1.0 for full right)
    pub brake: f32,                 // Amount of brake applied (0.0 to 1.0)
    pub clutch: u8,                 // Amount of clutch applied (0 to 100)
    pub gear: i8,                   // Gear selected (1-8, 0 = neutral, -1 = reverse)
    pub engine_rpm: u16,            // Engine RPM
    pub drs: u8,                    // 0 = off, 1 = on
    pub rev_lights_percent: u8,     // Rev lights indicator (percentage)
    pub rev_lights_bit_value: u16,  // Rev lights indicator (percentage)
    pub brake_temp: [u16; 4],       // Brake temperatures (FL, FR, RL, RR)
    pub tyre_surface_temp: [u8; 4], // Tyre surface temperatures
    pub tyre_inner_temp: [u8; 4],   // Tyre inner temperatures
    pub engine_temperature: u16,
    pub tyre_pressure: [f32; 4],
    pub surface_type: [u8; 4],
}

impl PacketSize for PacketCarTelemetry {
    fn size() -> usize {
        60
    }
}

fn read_f32_bytes(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
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
            rev_lights_bit_value: u16::from_le_bytes([bytes[20], bytes[21]]),
            brake_temp: [
                u16::from_le_bytes([bytes[22], bytes[23]]),
                u16::from_le_bytes([bytes[24], bytes[25]]),
                u16::from_le_bytes([bytes[26], bytes[27]]),
                u16::from_le_bytes([bytes[28], bytes[29]]),
            ],
            tyre_surface_temp: [bytes[30], bytes[31], bytes[32], bytes[33]],
            tyre_inner_temp: [bytes[34], bytes[35], bytes[36], bytes[37]],
            engine_temperature: u16::from_le_bytes([bytes[38], bytes[39]]),
            tyre_pressure: [
                read_f32_bytes(bytes, 40),
                read_f32_bytes(bytes, 44),
                read_f32_bytes(bytes, 48),
                read_f32_bytes(bytes, 52),
            ],
            surface_type: [bytes[56], bytes[57], bytes[58], bytes[59]],
        });
    }
}
