//! Here we register all the routes

use std::time::Duration;

use axum::Router;
use axum::extract::Request;
use axum::response::Response;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{Span, error, info, info_span};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::api::engine_health;
use crate::state::AppState;

/// Version prefix every API route lives under.
pub const API_PREFIX: &str = "/v1";
pub const OPENAPI_PATH: &str = "/v1/openapi.json";
pub const SWAGGER_UI_PATH: &str = "/swagger-ui";
pub const REDOC_PATH: &str = "/redoc";

/// The OpenAPI document. Paths are collected from the `#[utoipa::path]`
/// annotations on the handlers, so there is no second list to keep in sync.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Silo Engine API",
        description = "Sharpnr's self-hosted storage and infrastructure service."
    ),
    tags(
        (name = engine_health::TAG, description = "Engine health, version and status")
    )
)]
pub struct ApiDoc;

/// Assemble the whole HTTP surface: API routes, then the documentation UIs.
pub fn build(state: AppState) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(API_PREFIX, engine_health::router())
        .split_for_parts();

    // Per-request logging. Every request opens a span carrying the method and
    // path, so anything a handler logs is attributed to the request that
    // caused it. The completion line is emitted at INFO -- request logs are the
    // baseline signal for a service, not a debugging extra.
    let request_logging = TraceLayer::new_for_http()
        .make_span_with(|request: &Request| {
            info_span!(
                "request",
                method = %request.method(),
                path = %request.uri().path(),
            )
        })
        .on_request(())
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            info!(
                status = response.status().as_u16(),
                latency_ms = latency.as_secs_f64() * 1000.0,
                "request completed"
            );
        })
        .on_body_chunk(())
        .on_eos(())
        .on_failure(
            |failure: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                error!(
                    latency_ms = latency.as_secs_f64() * 1000.0,
                    "request failed: {failure}"
                );
            },
        );

    router
        .merge(SwaggerUi::new(SWAGGER_UI_PATH).url(OPENAPI_PATH, api.clone()))
        .merge(Redoc::with_url(REDOC_PATH, api))
        .with_state(state)
        .layer(request_logging)
}
