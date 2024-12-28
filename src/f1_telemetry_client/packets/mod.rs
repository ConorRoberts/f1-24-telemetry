pub mod session_data;

#[derive(Debug)]
pub enum PacketType {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
    TyreSets,
    MotionEx,
    TimeTrial,
}

impl TryFrom<u8> for PacketType {
    type Error = String;

    fn try_from(value: u8) -> Result<PacketType, String> {
        match value {
            0 => Ok(Self::Motion),
            1 => Ok(Self::Session),
            2 => Ok(Self::LapData),
            3 => Ok(Self::Event),
            4 => Ok(Self::Participants),
            5 => Ok(Self::CarSetups),
            6 => Ok(Self::CarTelemetry),
            7 => Ok(Self::CarStatus),
            8 => Ok(Self::FinalClassification),
            9 => Ok(Self::LobbyInfo),
            10 => Ok(Self::CarDamage),
            11 => Ok(Self::SessionHistory),
            12 => Ok(Self::TyreSets),
            13 => Ok(Self::MotionEx),
            14 => Ok(Self::TimeTrial),
            _ => Err(format!("Invalid packet type: {}", value)),
        }
    }
}
