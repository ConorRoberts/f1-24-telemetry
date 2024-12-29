use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct PacketMotionExData {
    // Note: All wheel arrays have the following order: RL, RR, FL, FR
    pub suspension_position: [f32; 4],     // Position of suspension
    pub suspension_velocity: [f32; 4],     // Velocity of suspension
    pub suspension_acceleration: [f32; 4], // Acceleration of suspension
    pub wheel_speed: [f32; 4],             // Speed of each wheel
    pub wheel_slip_ratio: [f32; 4],        // Slip ratio for each wheel
    pub wheel_slip_angle: [f32; 4],        // Slip angles for each wheel
    pub wheel_lat_force: [f32; 4],         // Lateral forces for each wheel
    pub wheel_long_force: [f32; 4],        // Longitudinal forces for each wheel
    pub height_of_cog_above_ground: f32,   // Height of centre of gravity above ground
    pub local_velocity_x: f32,             // Velocity in local space -- metres/s
    pub local_velocity_y: f32,             // Velocity in local space
    pub local_velocity_z: f32,             // Velocity in local space
    pub angular_velocity_x: f32,           // Angular velocity x-component -- radians/s
    pub angular_velocity_y: f32,           // Angular velocity y-component
    pub angular_velocity_z: f32,           // Angular velocity z-component
    pub angular_acceleration_x: f32,       // Angular acceleration x-component -- radians/s/s
    pub angular_acceleration_y: f32,       // Angular acceleration y-component
    pub angular_acceleration_z: f32,       // Angular acceleration z-component
    pub front_wheels_angle: f32,           // Current front wheels angle in radians
    pub wheel_vert_force: [f32; 4],        // Vertical forces for each wheel
    pub front_aero_height: f32,            // Front plank edge height above road surface
    pub rear_aero_height: f32,             // Rear plank edge height above road surface
    pub front_roll_angle: f32,             // Roll angle of the front suspension
    pub rear_roll_angle: f32,              // Roll angle of the rear suspension
    pub chassis_yaw: f32, // Yaw angle of the chassis relative to the direction of motion - radians
}

impl Packet for PacketMotionExData {
    fn size() -> usize {
        237 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketMotionExData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketMotionExData::size() {
            return Err("Packet too short for PacketMotionExData".into());
        }

        let mut offset = 0;

        // Helper function to read array of 4 f32 values
        let read_f32_array = |offset: &mut usize| -> [f32; 4] {
            let mut array = [0.0f32; 4];
            for i in 0..4 {
                array[i] = f32::from_le_bytes([
                    bytes[*offset],
                    bytes[*offset + 1],
                    bytes[*offset + 2],
                    bytes[*offset + 3],
                ]);
                *offset += 4;
            }
            array
        };

        // Helper function to read single f32 value
        let read_f32 = |offset: &mut usize| -> f32 {
            let value = f32::from_le_bytes([
                bytes[*offset],
                bytes[*offset + 1],
                bytes[*offset + 2],
                bytes[*offset + 3],
            ]);
            *offset += 4;
            value
        };

        // Read all wheel arrays
        let suspension_position = read_f32_array(&mut offset);
        let suspension_velocity = read_f32_array(&mut offset);
        let suspension_acceleration = read_f32_array(&mut offset);
        let wheel_speed = read_f32_array(&mut offset);
        let wheel_slip_ratio = read_f32_array(&mut offset);
        let wheel_slip_angle = read_f32_array(&mut offset);
        let wheel_lat_force = read_f32_array(&mut offset);
        let wheel_long_force = read_f32_array(&mut offset);

        // Read single values
        let height_of_cog_above_ground = read_f32(&mut offset);
        let local_velocity_x = read_f32(&mut offset);
        let local_velocity_y = read_f32(&mut offset);
        let local_velocity_z = read_f32(&mut offset);
        let angular_velocity_x = read_f32(&mut offset);
        let angular_velocity_y = read_f32(&mut offset);
        let angular_velocity_z = read_f32(&mut offset);
        let angular_acceleration_x = read_f32(&mut offset);
        let angular_acceleration_y = read_f32(&mut offset);
        let angular_acceleration_z = read_f32(&mut offset);
        let front_wheels_angle = read_f32(&mut offset);

        // Read final wheel array
        let wheel_vert_force = read_f32_array(&mut offset);

        // Read remaining single values
        let front_aero_height = read_f32(&mut offset);
        let rear_aero_height = read_f32(&mut offset);
        let front_roll_angle = read_f32(&mut offset);
        let rear_roll_angle = read_f32(&mut offset);
        let chassis_yaw = read_f32(&mut offset);

        Ok(PacketMotionExData {
            suspension_position,
            suspension_velocity,
            suspension_acceleration,
            wheel_speed,
            wheel_slip_ratio,
            wheel_slip_angle,
            wheel_lat_force,
            wheel_long_force,
            height_of_cog_above_ground,
            local_velocity_x,
            local_velocity_y,
            local_velocity_z,
            angular_velocity_x,
            angular_velocity_y,
            angular_velocity_z,
            angular_acceleration_x,
            angular_acceleration_y,
            angular_acceleration_z,
            front_wheels_angle,
            wheel_vert_force,
            front_aero_height,
            rear_aero_height,
            front_roll_angle,
            rear_roll_angle,
            chassis_yaw,
        })
    }
}
