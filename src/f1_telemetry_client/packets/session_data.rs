use super::Packet;

#[derive(Debug, Clone, Copy, Default)]
pub struct MarshalZone {
    pub zone_start: f32, // 0..1 fraction through lap
    pub zone_flag: i8,   // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
}

impl TryFrom<&[u8]> for MarshalZone {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < MarshalZone::size() {
            return Err("Buffer too small for MarshalZone".into());
        }

        return Ok(Self {
            zone_start: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            zone_flag: i8::from_le_bytes([bytes[4]]),
        });
    }
}

impl Packet for MarshalZone {
    fn size() -> usize {
        5
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WeatherForecastSample {
    pub session_type: u8,             // 0 = unknown, see appendix
    pub time_offset: u8,              // Time in minutes the forecast is for
    pub weather: u8, // 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8, // Track temp in Celsius
    pub track_temperature_change: i8, // 0 = up, 1 = down, 2 = no change
    pub air_temperature: i8, // Air temp in Celsius
    pub air_temperature_change: i8, // 0 = up, 1 = down, 2 = no change
    pub rain_percentage: u8, // Rain percentage (0-100)
}

impl Packet for WeatherForecastSample {
    fn size() -> usize {
        8
    }
}

impl TryFrom<&[u8]> for WeatherForecastSample {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < WeatherForecastSample::size() {
            return Err("Buffer too small for WeatherForecastSample".into());
        }

        return Ok(Self {
            session_type: bytes[0],
            time_offset: bytes[1],
            weather: bytes[2],
            track_temperature: i8::from_le_bytes([bytes[3]]),
            track_temperature_change: i8::from_le_bytes([bytes[4]]),
            air_temperature: i8::from_le_bytes([bytes[5]]),
            air_temperature_change: i8::from_le_bytes([bytes[6]]),
            rain_percentage: bytes[7],
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PacketSessionData {
    pub weather: u8, // 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
    pub track_temperature: i8, // Track temp in Celsius
    pub air_temperature: i8, // Air temp in Celsius
    pub total_laps: u8, // Total number of laps in race
    pub track_length: u16, // Track length in metres
    pub session_type: u8, // 0 = unknown, see appendix
    pub track_id: i8, // -1 for unknown, see appendix
    pub formula: u8, // 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic, 4 = Beta, 6 = Esports
    pub session_time_left: u16, // Time left in session in seconds
    pub session_duration: u16, // Session duration in seconds
    pub pit_speed_limit: u8, // Pit speed limit in km/h
    pub game_paused: u8, // 0 = not paused, 1 = paused
    pub is_spectating: u8, // 0 = not spectating, 1 = spectating
    pub spectator_car_index: u8, // Index of car being spectated
    pub sli_pro_native_support: u8, // 0 = inactive, 1 = active
    pub num_marshal_zones: u8, // Number of marshal zones
    pub marshal_zones: [MarshalZone; 21], // List of marshal zones
    pub safety_car_status: u8, // 0 = no safety car, 1 = full, 2 = virtual, 3 = formation lap
    pub network_game: u8, // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8,
    pub weather_forecast_samples: [WeatherForecastSample; 64],
    pub forecast_accuracy: u8,             // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,                 // AI Difficulty (0-110)
    pub season_link_identifier: u32,       // Season identifier
    pub weekend_link_identifier: u32,      // Weekend identifier
    pub session_link_identifier: u32,      // Session identifier
    pub pit_stop_window_ideal_lap: u8,     // Ideal pit stop lap
    pub pit_stop_window_latest_lap: u8,    // Latest pit stop lap
    pub pit_stop_rejoin_position: u8,      // Predicted rejoin position
    pub steering_assist: u8,               // 0 = off, 1 = on
    pub braking_assist: u8,                // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: u8,                // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: u8,                    // 0 = off, 1 = on
    pub pit_release_assist: u8,            // 0 = off, 1 = on
    pub ers_assist: u8,                    // 0 = off, 1 = on
    pub drs_assist: u8,                    // 0 = off, 1 = on
    pub dynamic_racing_line: u8,           // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: u8,      // 0 = 2D, 1 = 3D
    pub game_mode: u8,                     // Game mode id
    pub ruleset: u8,                       // Ruleset
    pub time_of_day: u32,                  // Minutes since midnight
    pub session_length: u8, // 0 = None, 2 = Very Short, 3 = Short, 4 = Medium, 5 = Medium Long, 6 = Long, 7 = Full
    pub speed_units_lead_player: u8, // 0 = MPH, 1 = KPH
    pub temperature_units_lead_player: u8, // 0 = Celsius, 1 = Fahrenheit
    pub speed_units_secondary_player: u8, // 0 = MPH, 1 = KPH
    pub temperature_units_secondary_player: u8, // 0 = Celsius, 1 = Fahrenheit
    pub num_safety_car_periods: u8,
    pub num_virtual_safety_car_periods: u8,
    pub num_red_flag_periods: u8,
    pub equal_car_performance: u8,             // 0 = Off, 1 = On
    pub recovery_mode: u8,                     // 0 = None, 1 = Flashbacks, 2 = Auto-recovery
    pub flashback_limit: u8,                   // 0 = Low, 1 = Medium, 2 = High, 3 = Unlimited
    pub surface_type: u8,                      // 0 = Simplified, 1 = Realistic
    pub low_fuel_mode: u8,                     // 0 = Easy, 1 = Hard
    pub race_starts: u8,                       // 0 = Manual, 1 = Assisted
    pub tyre_temperature: u8,                  // 0 = Surface only, 1 = Surface & Carcass
    pub pit_lane_tyre_sim: u8,                 // 0 = On, 1 = Off
    pub car_damage: u8,                        // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Simulation
    pub car_damage_rate: u8,                   // 0 = Reduced, 1 = Standard, 2 = Simulation
    pub collisions: u8,                        // 0 = Off, 1 = Player-to-Player Off, 2 = On
    pub collisions_off_for_first_lap_only: u8, // 0 = Disabled, 1 = Enabled
    pub mp_unsafe_pit_release: u8,             // 0 = On, 1 = Off (Multiplayer)
    pub mp_off_for_griefing: u8,               // 0 = Disabled, 1 = Enabled (Multiplayer)
    pub corner_cutting_stringency: u8,         // 0 = Regular, 1 = Strict
    pub parc_ferme_rules: u8,                  // 0 = Off, 1 = On
    pub pit_stop_experience: u8,               // 0 = Automatic, 1 = Broadcast, 2 = Immersive
    pub safety_car: u8,                        // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased
    pub safety_car_experience: u8,             // 0 = Broadcast, 1 = Immersive
    pub formation_lap: u8,                     // 0 = Off, 1 = On
    pub formation_lap_experience: u8,          // 0 = Broadcast, 1 = Immersive
    pub red_flags: u8,                         // 0 = Off, 1 = Reduced, 2 = Standard, 3 = Increased
    pub affects_licence_level_solo: u8,        // 0 = Off, 1 = On
    pub affects_licence_level_mp: u8,          // 0 = Off, 1 = On
    pub num_sessions_in_weekend: u8,
    pub weekend_structure: [u8; 12],     // List of session types
    pub sector2_lap_distance_start: f32, // Distance in m for sector 2 start
    pub sector3_lap_distance_start: f32, // Distance in m for sector 3 start
}

impl Packet for PacketSessionData {
    fn size() -> usize {
        724
    }
}

impl TryFrom<&[u8]> for PacketSessionData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketSessionData::size() {
            return Err("Buffer too small for PacketSessionData".into());
        }

        Ok(Self {
            weather: bytes[0],
            track_temperature: i8::from_le_bytes([bytes[1]]),
            air_temperature: i8::from_le_bytes([bytes[2]]),
            total_laps: bytes[3],
            track_length: u16::from_le_bytes([bytes[4], bytes[5]]),
            session_type: bytes[6],
            track_id: i8::from_le_bytes([bytes[7]]),
            formula: bytes[8],
            session_time_left: u16::from_le_bytes([bytes[9], bytes[10]]),
            session_duration: u16::from_le_bytes([bytes[11], bytes[12]]),
            pit_speed_limit: bytes[13],
            game_paused: bytes[14],
            is_spectating: bytes[15],
            spectator_car_index: bytes[16],
            sli_pro_native_support: bytes[17],
            num_marshal_zones: bytes[18],
            marshal_zones: {
                let mut zones = [MarshalZone::default(); 21];
                for i in 0..21 {
                    let base = 19 + (i * 5);
                    zones[i] = MarshalZone::try_from(&bytes[base..base + 5])?
                }
                zones
            },
            safety_car_status: bytes[124],
            network_game: bytes[125],
            num_weather_forecast_samples: bytes[126],
            weather_forecast_samples: {
                let mut samples = [WeatherForecastSample::default(); 64];
                for i in 0..64 {
                    let base = 127 + (i * 8);
                    samples[i] = WeatherForecastSample::try_from(&bytes[base..base + 8])?
                }
                samples
            },
            forecast_accuracy: bytes[639],
            ai_difficulty: bytes[640],
            season_link_identifier: u32::from_le_bytes([
                bytes[641], bytes[642], bytes[643], bytes[644],
            ]),
            weekend_link_identifier: u32::from_le_bytes([
                bytes[645], bytes[646], bytes[647], bytes[648],
            ]),
            session_link_identifier: u32::from_le_bytes([
                bytes[649], bytes[650], bytes[651], bytes[652],
            ]),
            pit_stop_window_ideal_lap: bytes[653],
            pit_stop_window_latest_lap: bytes[654],
            pit_stop_rejoin_position: bytes[655],
            steering_assist: bytes[656],
            braking_assist: bytes[657],
            gearbox_assist: bytes[658],
            pit_assist: bytes[659],
            pit_release_assist: bytes[660],
            ers_assist: bytes[661],
            drs_assist: bytes[662],
            dynamic_racing_line: bytes[663],
            dynamic_racing_line_type: bytes[664],
            game_mode: bytes[665],
            ruleset: bytes[666],
            time_of_day: u32::from_le_bytes([bytes[667], bytes[668], bytes[669], bytes[670]]),
            session_length: bytes[671],
            speed_units_lead_player: bytes[672],
            temperature_units_lead_player: bytes[673],
            speed_units_secondary_player: bytes[674],
            temperature_units_secondary_player: bytes[675],
            num_safety_car_periods: bytes[676],
            num_virtual_safety_car_periods: bytes[677],
            num_red_flag_periods: bytes[678],
            equal_car_performance: bytes[679],
            recovery_mode: bytes[680],
            flashback_limit: bytes[681],
            surface_type: bytes[682],
            low_fuel_mode: bytes[683],
            race_starts: bytes[684],
            tyre_temperature: bytes[685],
            pit_lane_tyre_sim: bytes[686],
            car_damage: bytes[687],
            car_damage_rate: bytes[688],
            collisions: bytes[689],
            collisions_off_for_first_lap_only: bytes[690],
            mp_unsafe_pit_release: bytes[691],
            mp_off_for_griefing: bytes[692],
            corner_cutting_stringency: bytes[693],
            parc_ferme_rules: bytes[694],
            pit_stop_experience: bytes[695],
            safety_car: bytes[696],
            safety_car_experience: bytes[697],
            formation_lap: bytes[698],
            formation_lap_experience: bytes[699],
            red_flags: bytes[700],
            affects_licence_level_solo: bytes[701],
            affects_licence_level_mp: bytes[702],
            num_sessions_in_weekend: bytes[703],
            weekend_structure: {
                let mut structure = [0u8; 12];
                structure.copy_from_slice(&bytes[704..716]);
                structure
            },
            sector2_lap_distance_start: f32::from_le_bytes([
                bytes[716], bytes[717], bytes[718], bytes[719],
            ]),
            sector3_lap_distance_start: f32::from_le_bytes([
                bytes[720], bytes[721], bytes[722], bytes[723],
            ]),
        })
    }
}
