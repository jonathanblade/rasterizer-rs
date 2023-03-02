use std::net::SocketAddr;

use poem::listener::TcpListener;
use poem::middleware::{AddData, Cors};
use poem::{EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

use crate::api::RasterizerApi;
use crate::config::RasterizerConfig;
use crate::context::RasterizerContext;
use crate::database::RasterizerDatabase;
use crate::error::CatchError;
use crate::types::RasterizerEnpoint;
use crate::webdriver::RasterizerWebDriver;

pub struct Rasterizer {
    pub app: RasterizerEnpoint,
}

impl Rasterizer {
    pub fn new(ctx: RasterizerContext, addr: SocketAddr) -> Self {
        let api = OpenApiService::new(
            RasterizerApi,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
        .server(format!("http://{}/api", addr));
        let ui = api.swagger_ui();
        let app = Route::new()
            .nest("/api", api)
            .nest("/api/docs", ui)
            .with(CatchError)
            .with(Cors::new())
            .with(AddData::new(ctx));
        Self { app }
    }
}

pub async fn bootstrap() {
    let cfg = RasterizerConfig::new();
    let db_pool = RasterizerDatabase::create_pool(cfg.db_url).await;
    let db = RasterizerDatabase::new(db_pool);
    let wd = RasterizerWebDriver::new(cfg.wd_url).await;
    let ctx = RasterizerContext::new(db, wd);
    let rasterizer = Rasterizer::new(ctx, cfg.server_addr);
    let listener = TcpListener::bind(cfg.server_addr);
    Server::new(listener)
        .run(rasterizer.app)
        .await
        .expect("Failed to run server");
}
