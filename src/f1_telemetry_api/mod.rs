// mod db;
mod routes;

use poem::{listener::TcpListener, Result, Route, Server};
use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use routes::{events::EventsApi, session::SessionApi};

#[derive(Object, Default)]
struct Something {
    hello: String,
    count: i32,
}

#[derive(ApiResponse)]
enum GetSomethingResponse {
    #[oai(status = 200)]
    Something(Json<Something>),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/something/:name", method = "get")]
    async fn hello(&self, Path(name): Path<String>) -> Result<GetSomethingResponse> {
        Ok(GetSomethingResponse::Something(Json(Something {
            hello: name,
            count: 0,
        })))
    }
}

#[derive(Default)]
pub struct F1TelemetryApi;

impl F1TelemetryApi {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start(&self, addr: &str) -> Result<()> {
        let events = EventsApi::new(500);

        events.start_listener("0.0.0.0:5000").await;

        let api_service = OpenApiService::new((Api, SessionApi, events), "Hello World", "1.0")
            .server(format!("http://{}", addr));

        let spec = api_service.spec_endpoint();

        let app = Route::new()
            .nest("/", api_service)
            .nest("/openapi.json", spec);

        println!("Program started. Visit: {}", addr);
        Server::new(TcpListener::bind(addr)).run(app).await.unwrap();

        return Ok(());
    }
}
