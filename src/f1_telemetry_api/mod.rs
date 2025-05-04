mod events;
mod routes;

use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use poem_openapi::OpenApiService;
use routes::events::EventsApi;
use tracing::info;

#[derive(Default)]
pub struct F1TelemetryApi;

impl F1TelemetryApi {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start(&self, addr: &str) -> Result<()> {
        let events = EventsApi::new(5000);

        // Begin listening for UDP data from F1 game
        events.start_listener("0.0.0.0:20777").await;

        let api_service =
            OpenApiService::new(events, "Hello World", "1.0").server(format!("http://{}", addr));

        let spec = api_service.spec_endpoint();

        let cors = Cors::new();

        let app = Route::new()
            .nest("/", api_service)
            .nest("/openapi.json", spec)
            .with(cors);

        info!("Program started. Visit: {}", addr);
        Server::new(TcpListener::bind(addr)).run(app).await.unwrap();

        return Ok(());
    }
}
