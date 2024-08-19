use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    normalize_path::{NormalizePath, NormalizePathLayer},
    trace::{self, TraceLayer},
};
use tower_layer::Layer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::random_humor_handler;
use crate::utils::response::VoidsongError;

use super::random_route;

#[derive(OpenApi)]
#[openapi(
    paths(random_humor_handler::chuck_norris, random_humor_handler::dad_joke),
    tags(
        (name = "humor", description = "Random jokes and humor."),
    )
)]
struct ApiDoc {}

pub fn routes() -> NormalizePath<Router> {
    let app_router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/random", random_route::routes())
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CompressionLayer::new());

    NormalizePathLayer::trim_trailing_slash().layer(app_router)
}

async fn handler_404() -> VoidsongError {
    VoidsongError::InvalidRoute
}
