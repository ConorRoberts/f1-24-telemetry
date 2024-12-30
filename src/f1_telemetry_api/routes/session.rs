use libsql::de;
use poem::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db::connect_db;

#[derive(Serialize, Deserialize, Object)]
struct Session {
    id: String,
}

#[derive(ApiResponse)]
enum GetSessionResponse {
    #[oai(status = 200)]
    Success(Json<Session>),
}

#[derive(Deserialize)]
struct SelectedCarTelemetry {
    session_id: String,
    timestamp: f32,
    throttle: f32,
    speed: u16,
    // steer: f32,
    // brake: f32,
    // clutch: u8,
    // gear: i8,
}

pub struct SessionApi;

#[OpenApi]
impl SessionApi {
    #[oai(path = "/:session_id", method = "get")]
    async fn get_session_data(&self, session_id: Path<String>) -> Result<GetSessionResponse> {
        let db = connect_db().await.unwrap();
        let mut rows = db.query("SELECT * FROM car_telemetry", ()).await.unwrap();

        while let Some(row) = rows.next().await.unwrap() {
            let value = de::from_row::<SelectedCarTelemetry>(&row).unwrap();
            // let id: i64 = row.get(0)?;
            // let name: String = row.get(1)?;
            info!(
                "Car Telemetry: Speed {}, Throttle {}",
                value.speed, value.throttle
            );
        }

        // info!("{}", rows.());

        return Ok(GetSessionResponse::Success(Json(Session {
            id: session_id.0,
        })));
    }
}
