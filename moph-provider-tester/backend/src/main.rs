mod handlers;
mod route;

use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use moph_provider_client::MophProviderIdClient;
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use time::{UtcOffset, format_description::well_known::Rfc3339};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{
    EnvFilter,
    fmt::time::OffsetTime,
    Layer,
    layer::SubscriberExt,
};

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<MophProviderIdClient>,
    pub nonce_set: Arc<Mutex<HashSet<String>>>,
}

#[tokio::main]
async fn main() {
    let timer = OffsetTime::new(UtcOffset::from_hms(7, 0, 0).unwrap_or(UtcOffset::UTC), Rfc3339);
    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_timer(timer)
                .with_ansi(true)
                .with_target(true)
                .with_filter(EnvFilter::new("debug,hyper=warn,axum=trace")),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    info!("Start logging");

    let handle_404 = handle_404.into_service();
    let root_dir = ServeDir::new("volume/pwa")
        .precompressed_br()
        .precompressed_gzip()
        // .precompressed_deflate()
        // .precompressed_zstd()
        .not_found_service(handle_404);

    let client = std::sync::Arc::new(MophProviderIdClient::builder()
        // call only one redirect_uri() 
        .redirect_uri("http://localhost:8088/provider/fast")
        // .redirect_uri("http://localhost:8088/provider/full")
        .health_id_client(include_str!("credentials/health-id-client.txt"))
        .health_id_key(include_str!("credentials/health-id-secret.txt"))
        .provider_id_client(include_str!("credentials/provider-id-client.txt"))
        .provider_id_key(include_str!("credentials/provider-id-secret.txt"))
        .build()
        .await
        .unwrap()
    );

    let state = AppState {
        client,
        nonce_set: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = Router::new()
        .nest("/api", route::router(state.clone()))
        .nest("/provider", route::provider_router(state.clone()))
        .fallback_service(root_dir);
    serve_http(8088, app).await;
}

async fn serve_http(port: u16, app: Router) {
    let http_addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
    info!(
        "HTTP server started listening on {}, please Ctrl-c to terminate server.",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}