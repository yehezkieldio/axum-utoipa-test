use crate::utils::response::VoidsongError;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};
use axum::{Extension, Json, Router};
use tower_http::{
    compression::CompressionLayer,
    normalize_path::{NormalizePath, NormalizePathLayer},
    trace::{self, TraceLayer},
};
use tower_layer::Layer;
use tracing::Level;

use super::random_route;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

pub fn routes() -> NormalizePath<Router> {
    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let app_router = ApiRouter::new()
        .route("/api.json", get(serve_api))
        .nest("/random", random_route::routes())
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CompressionLayer::new())
        .finish_api(&mut api)
        .layer(Extension(api));

    NormalizePathLayer::trim_trailing_slash().layer(app_router)
}

async fn handler_404() -> VoidsongError {
    VoidsongError::InvalidRoute
}
