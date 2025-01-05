use crate::f1_telemetry_client::TelemetryPacket;
use poem_openapi::{Enum, Object, Union};
use serde::{Deserialize, Serialize};

// Heartbeat event
#[derive(Object, Clone, Debug)]
pub struct HeartbeatEvent {
    #[oai(rename = "type")]
    pub event_type: EventType,
}

#[derive(Clone, Enum, Serialize, Deserialize, Debug)]
#[oai(rename = "EventType")]
pub enum EventType {
    #[oai(rename = "car_telemetry")]
    CarTelemetryEvent,
    #[oai(rename = "car_motion")]
    CarMotionEvent,
    #[oai(rename = "lap_data")]
    LapDataEvent,
    #[oai(rename = "heartbeat")]
    Heartbeat,
}

// Union type for all possible events
#[derive(Union, Clone, Debug)]
#[oai(discriminator_name = "type", one_of)]
pub enum Event {
    #[oai(mapping = "car_telemetry")]
    CarTelemetry(CarTelemetryEvent),
    #[oai(mapping = "car_motion")]
    CarMotion(CarMotionEvent),
    #[oai(mapping = "lap_data")]
    LapData(LapDataEvent),
    #[oai(mapping = "heartbeat")]
    Heartbeat(HeartbeatEvent),
}

// Data event
#[derive(Object, Clone, Debug)]
pub struct CarTelemetryEvent {
    #[oai(rename = "type")]
    pub event_type: EventType,
    pub throttle: f32,
    pub brake: f32,
    pub speed: u16,
    pub brake_temp: [u16; 4],       // Brake temperatures (FL, FR, RL, RR)
    pub tyre_surface_temp: [u8; 4], // Tyre surface temperatures
    pub tyre_inner_temp: [u8; 4],   // Tyre inner temperatures
    pub engine_temperature: u16,
    pub tyre_pressure: [f32; 4],
}

#[derive(Object, Clone, Debug)]
pub struct CarMotionEvent {
    #[oai(rename = "type")]
    pub event_type: EventType,
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
}

#[derive(Object, Clone, Debug)]
pub struct LapDataEvent {
    #[oai(rename = "type")]
    pub event_type: EventType,
    pub last_lap_time_in_ms: u32,
    pub current_lap_time_in_ms: u32,
    pub sector1_time_ms_part: u16,
    pub sector1_time_minutes_part: u8,
    pub sector2_time_ms_part: u16,
    pub sector2_time_minutes_part: u8,
    pub delta_to_car_in_front_ms_part: u16,
    pub delta_to_car_in_front_minutes_part: u8,
    pub delta_to_race_leader_ms_part: u16,
    pub delta_to_race_leader_minutes_part: u8,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub sector: u8,
    pub current_lap_invalid: u8,
    pub grid_position: u8,
    pub driver_status: u8,
    pub result_status: u8,
}

impl TryFrom<TelemetryPacket> for Event {
    type Error = String;

    fn try_from(value: TelemetryPacket) -> Result<Event, Self::Error> {
        match value {
            TelemetryPacket::CarTelemetry((_, data)) => {
                Ok(Event::CarTelemetry(CarTelemetryEvent {
                    event_type: EventType::CarTelemetryEvent,
                    brake: data.brake,
                    throttle: data.throttle,
                    speed: data.speed,
                    tyre_inner_temp: data.tyre_inner_temp,
                    tyre_surface_temp: data.tyre_surface_temp,
                    brake_temp: data.brake_temp,
                    engine_temperature: data.engine_temperature,
                    tyre_pressure: data.tyre_pressure,
                }))
            }
            TelemetryPacket::Motion((_, data)) => match data.car_motion_data.get(0) {
                Some(m) => Ok(Event::CarMotion(CarMotionEvent {
                    event_type: EventType::CarTelemetryEvent,
                    g_force_lateral: m.g_force_lateral,
                    g_force_longitudinal: m.g_force_longitudinal,
                    g_force_vertical: m.g_force_vertical,
                    world_position_x: m.world_position_x,
                    world_position_y: m.world_position_y,
                    world_position_z: m.world_position_z,
                })),
                _ => Err("Could not get car data for first car".into()),
            },
            TelemetryPacket::LapData((_, data)) => match data.lap_data.get(0) {
                Some(d) => Ok(Event::LapData(LapDataEvent {
                    event_type: EventType::LapDataEvent,
                    car_position: d.car_position,
                    current_lap_invalid: d.current_lap_invalid,
                    current_lap_num: d.current_lap_num,
                    current_lap_time_in_ms: d.current_lap_time_in_ms,
                    delta_to_car_in_front_minutes_part: d.delta_to_car_in_front_minutes_part,
                    delta_to_car_in_front_ms_part: d.delta_to_car_in_front_ms_part,
                    delta_to_race_leader_minutes_part: d.delta_to_race_leader_minutes_part,
                    delta_to_race_leader_ms_part: d.delta_to_race_leader_ms_part,
                    driver_status: d.driver_status,
                    grid_position: d.grid_position,
                    lap_distance: d.lap_distance,
                    last_lap_time_in_ms: d.last_lap_time_in_ms,
                    result_status: d.result_status,
                    sector: d.sector,
                    sector1_time_minutes_part: d.sector1_time_minutes_part,
                    sector1_time_ms_part: d.sector1_time_ms_part,
                    sector2_time_minutes_part: d.sector2_time_minutes_part,
                    sector2_time_ms_part: d.sector2_time_ms_part,
                    total_distance: d.total_distance,
                })),
                _ => Err("Error getting first item of lap data array".into()),
            },
            _ => Err("Unimplemented".into()),
        }
    }
}
