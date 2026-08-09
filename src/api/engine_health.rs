//! endpoints for engine related stuff like
//! health, version, status, etc

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// OpenAPI tag these endpoints are grouped under.
pub const TAG: &str = "engine";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(health))
        .routes(routes!(ready))
        .routes(routes!(version))
        .routes(routes!(status))
}

/// Liveness: the process is up and able to answer.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Always `ok` -- if the process cannot answer, nothing is returned.
    #[schema(example = "ok")]
    pub status: String,
}

/// Readiness: the process is up *and* its dependencies are usable.
#[derive(Serialize, ToSchema)]
pub struct ReadinessResponse {
    /// `ready` or `not_ready`.
    #[schema(example = "ready")]
    pub status: String,
    /// One entry per dependency Silo needs before it can serve traffic.
    pub checks: Vec<ComponentStatus>,
}

/// The state of a single dependency.
#[derive(Serialize, ToSchema)]
pub struct ComponentStatus {
    /// Dependency name, e.g. `postgres` or `storage`.
    pub name: String,
    /// `up` or `down`.
    pub status: String,
    /// Why the dependency is down, when it is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Build and environment identity.
#[derive(Serialize, ToSchema)]
pub struct VersionResponse {
    #[schema(example = "silo-engine")]
    pub name: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    /// Value of `APP_ENV` this process started with.
    #[schema(example = "development")]
    pub environment: String,
}

/// Everything the health and version endpoints report, in one call.
#[derive(Serialize, ToSchema)]
pub struct StatusResponse {
    #[schema(example = "silo-engine")]
    pub name: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    #[schema(example = "development")]
    pub environment: String,
    #[schema(example = "ok")]
    pub status: String,
    /// Seconds since the process started serving.
    #[schema(example = 42)]
    pub uptime_seconds: u64,
}

/// Liveness probe.
///
/// Answers as long as the process is running. Never touches a dependency, so
/// a restart supervisor does not kill Silo because PostgreSQL is slow.
#[utoipa::path(
    get,
    path = "/health",
    tag = TAG,
    responses(
        (status = OK, description = "Process is alive", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

/// Readiness probe.
///
/// Reports whether Silo can actually serve traffic. Returns `503` when any
/// dependency is down, so a load balancer can take this instance out of
/// rotation without restarting it.
#[utoipa::path(
    get,
    path = "/health/ready",
    tag = TAG,
    responses(
        (status = OK, description = "All dependencies are usable", body = ReadinessResponse),
        (status = SERVICE_UNAVAILABLE, description = "A dependency is down", body = ReadinessResponse)
    )
)]
pub async fn ready() -> (StatusCode, Json<ReadinessResponse>) {
    // Silo has no external dependencies wired up yet. Each one gets a
    // ComponentStatus pushed here as it lands (PostgreSQL metadata, storage
    // backend, auth service), and the overall status follows from them.
    let checks: Vec<ComponentStatus> = Vec::new();

    let all_up = checks.iter().all(|check| check.status == "up");
    let (code, status) = if all_up {
        (StatusCode::OK, "ready")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not_ready")
    };

    (
        code,
        Json(ReadinessResponse {
            status: status.to_string(),
            checks,
        }),
    )
}

/// Build and environment identity.
#[utoipa::path(
    get,
    path = "/version",
    tag = TAG,
    responses(
        (status = OK, description = "Build and environment identity", body = VersionResponse)
    )
)]
pub async fn version(State(state): State<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.app_env.clone(),
    })
}

/// Combined status.
#[utoipa::path(
    get,
    path = "/status",
    tag = TAG,
    responses(
        (status = OK, description = "Combined health, build and uptime", body = StatusResponse)
    )
)]
pub async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.app_env.clone(),
        status: "ok".to_string(),
        uptime_seconds: state.uptime_seconds(),
    })
}
