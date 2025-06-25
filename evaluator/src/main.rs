mod eval;
mod links;
mod nsjail;
mod spec;

use axum::extract::MatchedPath;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use dotenv::dotenv;
use eval::{eval_handler, initialize_evaluator};
use metrics::{describe_counter, describe_gauge};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use log::error;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use std::sync::Arc;

use axum::{routing::get, routing::post, Router};

use anyhow::Result;

use lazy_static::lazy_static;

struct AppState {
    redis: redis::aio::ConnectionManager,
}

/// Custom error type for the application
/// If the status code is 5xx, then the error is logged
/// and the message is not reported to the client
struct AppError {
    err: anyhow::Error,
    status_code: Option<StatusCode>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (match self {
            AppError {
                status_code: Some(status_code),
                err,
            } => (status_code, err.to_string()),
            AppError {
                status_code: None,
                err,
            } => {
                error!("{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        },)
            .into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError {
            err,
            status_code: None,
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError {
            err: err.into(),
            status_code: None,
        }
    }
}

lazy_static! {
    static ref PROMETHEUS: PrometheusHandle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to create PrometheusBuilder");
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    initialize_evaluator().await?;

    describe_gauge!(
        "http_connections_active_total",
        "Total number of active connections"
    );
    // Evaluator
    describe_counter!(
        "evaluator_requests_total",
        "Number of requests to the evaluator"
    );
    describe_counter!(
        "evaluator_errors_total",
        "Total number of errors (panics) in the evaluator"
    );
    describe_counter!(
        "submitted_program_errors_total",
        "Number of errors in the user submitted program"
    );

    // Links
    describe_counter!(
        "link_new_requests_total",
        "Number of requests to the link service"
    );

    let redis_url = std::env::var("REDIS_LINKS_HOST").expect("REDIS_LINKS_HOST is not set");
    let redis_client = redis::Client::open(redis_url.clone())?;
    let redis = redis::aio::ConnectionManager::new(redis_client).await?;

    let rate_limit_links = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10 * 60)
            .burst_size(links::BURST_SIZE)
            .key_extractor(SmartIpKeyExtractor {})
            .error_handler(|e| match e {
                tower_governor::GovernorError::TooManyRequests { wait_time, .. } => {
                    Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", wait_time)
                        .body("Too many requests".into())
                        .expect("Failed to create rate limit error")
                }
                tower_governor::GovernorError::UnableToExtractKey => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Unable to find X-Forwarded-For or X-Real-Ip".into())
                    .expect("Failed to create rate limit error"),
                tower_governor::GovernorError::Other { .. } => unreachable!(),
            })
            .finish()
            .expect("Failed to create rate limiter"),
    );

    let rate_limit_evaluate = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(3)
            .burst_size(30)
            .key_extractor(SmartIpKeyExtractor {})
            .error_handler(|e| {
                log::error!("Rate limit evaluator error: {:?}", e);
                match e {
                    tower_governor::GovernorError::TooManyRequests { wait_time, .. } => {
                        Response::builder()
                            .status(StatusCode::TOO_MANY_REQUESTS)
                            .header("Retry-After", wait_time)
                            .body("Too many requests".into())
                            .expect("Failed to create rate limit error")
                    }
                    tower_governor::GovernorError::UnableToExtractKey => Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Unable to find X-Forwarded-For or X-Real-Ip".into())
                        .expect("Failed to create rate limit error"),
                    tower_governor::GovernorError::Other { .. } => unreachable!(),
                }
            })
            .finish()
            .expect("Failed to create rate limiter"),
    );

    let state = Arc::new(AppState { redis });

    let app = Router::new()
        .route(
            "/api/v1/evaluate",
            post(eval_handler).layer(GovernorLayer {
                config: rate_limit_evaluate,
            }),
        )
        .route(
            "/api/v1/link/new",
            post(links::new_handler).layer(GovernorLayer {
                config: rate_limit_links,
            }),
        )
        .route("/api/v1/link/get/:key", get(links::get_handler))
        .route("/metrics", get(|| async { PROMETHEUS.render() }))
        .route("/liveness", get(|| async { "OK" }))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    "x-real-ip" = request
                        .headers()
                        .get("X-Real-Ip")
                        .and_then(|h| h.to_str().ok())
                        .unwrap_or("unknown"),
                    "x-forwarded-for" = request
                        .headers()
                        .get("X-Forwarded-For")
                        .and_then(|h| h.to_str().ok())
                        .unwrap_or("unknown"),
                )
            }),
        )
        .with_state(state);

    tracing::info!("Starting server on 0.0.0.0:7800");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7800").await?;
    Ok(axum::serve(listener, app).await?)
}

#[cfg(test)]
mod test {}
