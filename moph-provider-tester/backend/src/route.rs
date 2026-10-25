use axum::{
    body::Body,
    http::{
        header::{self, HeaderValue},
        Request, StatusCode,
    },
    routing::get,
    Router,
};
use std::time::Duration;
use tower_http::{
    compression::{
        predicate::{NotForContentType, Predicate, SizeAbove},
        CompressionLayer,
    },
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Level;

use crate::handlers;

pub fn router(state: crate::AppState) -> Router {
    let compression_predicate = SizeAbove::new(1024)
        // SSE *MUST NOT* COMPRESS, if compressed, data will send once when closed
        .and(NotForContentType::const_new("text/event-stream"))
        // image and woff file already compressed
        .and(NotForContentType::IMAGES)
        .and(NotForContentType::const_new("font/woff"))
        .and(NotForContentType::const_new("font/woff2"));

    Router::new()
        .route("/greet", get(handlers::greet_handler))
        .route("/oauth", get(handlers::get_oauth_uri_handler))
        .route("/oauth-nonce", get(handlers::get_oauth_uri_with_nonce_handler))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                tracing::span!(
                    Level::ERROR,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(ulid::Ulid::generate()),
                )
            }),
        )
        .layer(CompressionLayer::new().compress_when(compression_predicate))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
}

pub fn provider_router(state: crate::AppState) -> Router {
    Router::new()
        .route("/fast", axum::routing::get(handlers::provider_auth_fast))
        .route("/full", axum::routing::get(handlers::provider_auth_full))
        .with_state(state.clone())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                tracing::span!(
                    Level::ERROR,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(ulid::Ulid::generate()),
                )
            }),
        )
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(1024)))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
}
