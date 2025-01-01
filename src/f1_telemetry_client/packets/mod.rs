pub mod car_damage;
pub mod car_motion_data;
pub mod car_setups;
pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod final_classification;
pub mod header;
pub mod lap_data;
pub mod lobby_info;
pub mod motion_ex;
pub mod participants;
pub mod session_data;
pub mod time_trial;
pub mod tyre_sets;

pub trait PacketSize {
    fn size() -> usize;
}
