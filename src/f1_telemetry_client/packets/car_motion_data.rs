use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct CarMotionData {
    // World space X position - metres
    pub world_position_x: f32,
    // World space Y position
    pub world_position_y: f32,
    // World space Z position
    pub world_position_z: f32,
    // Velocity in world space X – metres/s
    pub world_velocity_x: f32,
    // Velocity in world space Y
    pub world_velocity_y: f32,
    // Velocity in world space Z
    pub world_velocity_z: f32,
    // World space forward X direction (normalised)
    pub world_forward_dir_x: i16,
    // World space forward Y direction (normalised)
    pub world_forward_dir_y: i16,
    // World space forward Z direction (normalised)
    pub world_forward_dir_z: i16,
    // World space right X direction (normalised)
    pub world_right_dir_x: i16,
    // World space right Y direction (normalised)
    pub world_right_dir_y: i16,
    // World space right Z direction (normalised)
    pub world_right_dir_z: i16,
    // Lateral G-Force component
    pub g_force_lateral: f32,
    // Longitudinal G-Force component
    pub g_force_longitudinal: f32,
    // Vertical G-Force component
    pub g_force_vertical: f32,
    // Yaw angle in radians
    pub yaw: f32,
    // Pitch angle in radians
    pub pitch: f32,
    // Roll angle in radians
    pub roll: f32,
}

impl Packet for CarMotionData {
    fn size() -> usize {
        60
    }
}

#[derive(Debug, Clone)]
pub struct PacketMotionData {
    // Data for all cars on track
    pub car_motion_data: [CarMotionData; 22],
}

impl Packet for PacketMotionData {
    fn size() -> usize {
        CarMotionData::size() * 22
    }
}

impl TryFrom<&[u8]> for CarMotionData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < CarMotionData::size() {
            return Err("Buffer too small for CarMotionData".into());
        }

        Ok(Self {
            world_position_x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            world_position_y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            world_position_z: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            world_velocity_x: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            world_velocity_y: f32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            world_velocity_z: f32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            world_forward_dir_x: i16::from_le_bytes([bytes[24], bytes[25]]),
            world_forward_dir_y: i16::from_le_bytes([bytes[26], bytes[27]]),
            world_forward_dir_z: i16::from_le_bytes([bytes[28], bytes[29]]),
            world_right_dir_x: i16::from_le_bytes([bytes[30], bytes[31]]),
            world_right_dir_y: i16::from_le_bytes([bytes[32], bytes[33]]),
            world_right_dir_z: i16::from_le_bytes([bytes[34], bytes[35]]),
            g_force_lateral: f32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
            g_force_longitudinal: f32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]),
            g_force_vertical: f32::from_le_bytes([bytes[44], bytes[45], bytes[46], bytes[47]]),
            yaw: f32::from_le_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]),
            pitch: f32::from_le_bytes([bytes[52], bytes[53], bytes[54], bytes[55]]),
            roll: f32::from_le_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]),
        })
    }
}

impl TryFrom<&[u8]> for PacketMotionData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketMotionData::size() {
            return Err("Buffer too small for PacketMotionData".into());
        }

        let mut car_motion_data = [CarMotionData::default(); 22];
        for i in 0..22 {
            let start = i * CarMotionData::size();
            car_motion_data[i] =
                CarMotionData::try_from(&bytes[start..start + size_of::<CarMotionData>()])?;
        }

        Ok(Self { car_motion_data })
    }
}
