use super::Packet;

#[derive(Debug, Clone)]
pub struct LobbyInfoData {
    pub ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub team_id: u8,       // Team id - see appendix (255 if no team currently selected)
    pub nationality: u8,   // Nationality of the driver
    pub platform: u8,      // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
    pub name: String,      // Name of participant in UTF-8 format -- null terminated
    // Will be truncated with ... (U+2026) if too long
    pub car_number: u8,        // Car number of the player
    pub your_telemetry: u8,    // The player's UDP setting, 0 = restricted, 1 = public
    pub show_online_names: u8, // The player's show online names setting, 0 = off, 1 = on
    pub tech_level: u16,       // F1 World tech level
    pub ready_status: u8,      // 0 = not ready, 1 = ready, 2 = spectating
}

#[derive(Debug, Clone)]
pub struct PacketLobbyInfoData {
    pub num_players: u8,                   // Number of players in the lobby data
    pub lobby_players: Vec<LobbyInfoData>, // Data for all players in the lobby
}

impl Packet for PacketLobbyInfoData {
    fn size() -> usize {
        1306 // Size specified in the UDP spec
    }
}

impl TryFrom<&[u8]> for PacketLobbyInfoData {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < PacketLobbyInfoData::size() {
            return Err("Packet too short for PacketLobbyInfoData".into());
        }

        let num_players = bytes[0];
        let mut lobby_players = Vec::with_capacity(22);
        let mut offset = 1;

        // Parse lobby data for each player
        for _ in 0..22 {
            // Process name as UTF-8 string with null termination
            let name_bytes = &bytes[offset + 4..offset + 52]; // 48 bytes for name
            let name_end = name_bytes.iter().position(|&x| x == 0).unwrap_or(48);
            let name = String::from_utf8(name_bytes[..name_end].to_vec())
                .map_err(|_| "Invalid UTF-8 in player name")?;

            let data = LobbyInfoData {
                ai_controlled: bytes[offset],
                team_id: bytes[offset + 1],
                nationality: bytes[offset + 2],
                platform: bytes[offset + 3],
                name,
                car_number: bytes[offset + 52],
                your_telemetry: bytes[offset + 53],
                show_online_names: bytes[offset + 54],
                tech_level: u16::from_le_bytes([bytes[offset + 55], bytes[offset + 56]]),
                ready_status: bytes[offset + 57],
            };
            lobby_players.push(data);
            offset += 58; // Size of each lobby player data block
        }

        Ok(PacketLobbyInfoData {
            num_players,
            lobby_players,
        })
    }
}

impl Default for LobbyInfoData {
    fn default() -> Self {
        LobbyInfoData {
            ai_controlled: 0,
            team_id: 255,
            nationality: 0,
            platform: 255,
            name: String::new(),
            car_number: 0,
            your_telemetry: 0,
            show_online_names: 0,
            tech_level: 0,
            ready_status: 0,
        }
    }
}

impl Default for PacketLobbyInfoData {
    fn default() -> Self {
        PacketLobbyInfoData {
            num_players: 0,
            lobby_players: Vec::new(),
        }
    }
}
