use super::PacketSize;

#[derive(Debug, Clone)]
pub struct ParticipantData {
    pub ai_controlled: u8,  // Whether the vehicle is AI (1) or Human (0) controlled
    pub driver_id: u8,      // Driver id - see appendix, 255 if network human
    pub network_id: u8,     // Network id - unique identifier for network players
    pub team_id: u8,        // Team id - see appendix
    pub my_team: u8,        // My team flag - 1 = My Team, 0 = otherwise
    pub race_number: u8,    // Race number of the car
    pub nationality: u8,    // Nationality of the driver
    pub name: String,       // Name of participant in UTF-8 format
    pub your_telemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
    pub show_online_names: u8, // The player's show online names setting, 0 = off, 1 = on
    pub tech_level: u16,    // F1 World tech level
    pub platform: u8,       // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
}

#[derive(Debug, Clone)]
pub struct PacketParticipantsData {
    pub num_active_cars: u8,                // Number of active cars in the data
    pub participants: Vec<ParticipantData>, // List of participants
}

impl PacketSize for PacketParticipantsData {
    fn size() -> usize {
        1350 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketParticipantsData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketParticipantsData::size() {
            return Err("Packet too short for PacketParticipantsData".into());
        }

        let num_active_cars = bytes[0];
        let mut offset = 1;
        let mut participants = Vec::with_capacity(22);

        for _ in 0..22 {
            // Each participant data structure is fixed size
            let name_bytes = &bytes[offset + 7..offset + 55]; // 48 bytes for name
            let name_end = name_bytes.iter().position(|&x| x == 0).unwrap_or(48);
            let name = String::from_utf8(name_bytes[..name_end].to_vec())
                .map_err(|_| "Invalid UTF-8 in participant name")?;

            participants.push(ParticipantData {
                ai_controlled: bytes[offset],
                driver_id: bytes[offset + 1],
                network_id: bytes[offset + 2],
                team_id: bytes[offset + 3],
                my_team: bytes[offset + 4],
                race_number: bytes[offset + 5],
                nationality: bytes[offset + 6],
                name,
                your_telemetry: bytes[offset + 55],
                show_online_names: bytes[offset + 56],
                tech_level: u16::from_le_bytes([bytes[offset + 57], bytes[offset + 58]]),
                platform: bytes[offset + 59],
            });

            offset += 60; // Size of each participant's data block
        }

        Ok(PacketParticipantsData {
            num_active_cars,
            participants,
        })
    }
}

// Optional: Implement Default if needed
impl Default for ParticipantData {
    fn default() -> Self {
        ParticipantData {
            ai_controlled: 0,
            driver_id: 255,
            network_id: 0,
            team_id: 0,
            my_team: 0,
            race_number: 0,
            nationality: 0,
            name: String::new(),
            your_telemetry: 0,
            show_online_names: 0,
            tech_level: 0,
            platform: 255,
        }
    }
}

impl Default for PacketParticipantsData {
    fn default() -> Self {
        PacketParticipantsData {
            num_active_cars: 0,
            participants: Vec::new(),
        }
    }
}
