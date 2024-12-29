use super::Packet;

#[derive(Debug, Clone, Copy)]
pub struct FastestLap {
    pub vehicle_idx: u8,
    pub lap_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Retirement {
    pub vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct TeamMateInPits {
    pub vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct RaceWinner {
    pub vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Penalty {
    pub penalty_type: u8,
    pub infringement_type: u8,
    pub vehicle_idx: u8,
    pub other_vehicle_idx: u8,
    pub time: u8,
    pub lap_num: u8,
    pub places_gained: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,
    pub speed: f32,
    pub is_overall_fastest_in_session: u8,
    pub is_driver_fastest_in_session: u8,
    pub fastest_vehicle_idx_in_session: u8,
    pub fastest_speed_in_session: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct StartLights {
    pub num_lights: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct DriveThroughPenaltyServed {
    pub vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct StopGoPenaltyServed {
    pub vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Flashback {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Buttons {
    pub button_status: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Overtake {
    pub overtaking_vehicle_idx: u8,
    pub being_overtaken_vehicle_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct SafetyCar {
    pub safety_car_type: u8,
    pub event_type: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub vehicle1_idx: u8,
    pub vehicle2_idx: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum EventDataDetails {
    FastestLap(FastestLap),
    Retirement(Retirement),
    TeamMateInPits(TeamMateInPits),
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    Flashback(Flashback),
    Buttons(Buttons),
    Overtake(Overtake),
    SafetyCar(SafetyCar),
    Collision(Collision),
}

#[derive(Debug, Clone, Copy)]
pub struct PacketEventData {
    pub event_string_code: [u8; 4],
    pub event_details: EventDataDetails,
}

impl Packet for PacketEventData {
    fn size() -> usize {
        45  // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketEventData {
    type Error = String;
    
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketEventData::size() {
            return Err("Packet too short for PacketEventData".into());
        }

        let event_string_code = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let event_code = std::str::from_utf8(&event_string_code)
            .map_err(|_| "Invalid event string code")?;

        let event_details = match event_code {
            "FTLP" => EventDataDetails::FastestLap(FastestLap {
                vehicle_idx: bytes[4],
                lap_time: f32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]),
            }),
            "RTMT" => EventDataDetails::Retirement(Retirement {
                vehicle_idx: bytes[4],
            }),
            "TMPT" => EventDataDetails::TeamMateInPits(TeamMateInPits {
                vehicle_idx: bytes[4],
            }),
            "RCWN" => EventDataDetails::RaceWinner(RaceWinner {
                vehicle_idx: bytes[4],
            }),
            "PENA" => EventDataDetails::Penalty(Penalty {
                penalty_type: bytes[4],
                infringement_type: bytes[5],
                vehicle_idx: bytes[6],
                other_vehicle_idx: bytes[7],
                time: bytes[8],
                lap_num: bytes[9],
                places_gained: bytes[10],
            }),
            "SPTP" => EventDataDetails::SpeedTrap(SpeedTrap {
                vehicle_idx: bytes[4],
                speed: f32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]),
                is_overall_fastest_in_session: bytes[9],
                is_driver_fastest_in_session: bytes[10],
                fastest_vehicle_idx_in_session: bytes[11],
                fastest_speed_in_session: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            }),
            "STLG" => EventDataDetails::StartLights(StartLights {
                num_lights: bytes[4],
            }),
            "DTSV" => EventDataDetails::DriveThroughPenaltyServed(DriveThroughPenaltyServed {
                vehicle_idx: bytes[4],
            }),
            "SGSV" => EventDataDetails::StopGoPenaltyServed(StopGoPenaltyServed {
                vehicle_idx: bytes[4],
            }),
            "FLBK" => EventDataDetails::Flashback(Flashback {
                flashback_frame_identifier: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
                flashback_session_time: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            }),
            "BUTN" => EventDataDetails::Buttons(Buttons {
                button_status: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            }),
            "OVTK" => EventDataDetails::Overtake(Overtake {
                overtaking_vehicle_idx: bytes[4],
                being_overtaken_vehicle_idx: bytes[5],
            }),
            "SCAR" => EventDataDetails::SafetyCar(SafetyCar {
                safety_car_type: bytes[4],
                event_type: bytes[5],
            }),
            "COLL" => EventDataDetails::Collision(Collision {
                vehicle1_idx: bytes[4],
                vehicle2_idx: bytes[5],
            }),
            _ => return Err(format!("Unknown event code: {}", event_code)),
        };

        Ok(PacketEventData {
            event_string_code,
            event_details,
        })
    }
}