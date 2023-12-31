// Importing the necessary modules and services.
use std::sync::Arc;
use anyhow::Context;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use trapziu_backend::config::{CONFIG, CONTEXT};
use trapziu_backend::prisma::prisma::PrismaClient;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
use axum::http::StatusCode;
use axum::{ BoxError, Extension, Json};
use axum::error_handling::HandleErrorLayer;


// The main function of the application.
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Setting up the tracing subscriber with the log level from the configuration.
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&CONFIG.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Creating a new Prisma client.
    let prisma_client = Arc::new(PrismaClient::_builder().build().await?);

    // Setting up CORS with the `CorsLayer`.
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    // Creating the application with the defined routes and middleware.
    let app = trapziu_backend::service::Router::new()
        .layer(cors)
        .layer(Extension(prisma_client))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(std::time::Duration::from_secs(30)),
        )
        .with_state(CONTEXT.clone());

    // Logging the start of the server.
    tracing::info!("starting server on port {}", CONFIG.backend_port);

    // Binding the server to the configured port and starting it.
    let listener = tokio::net::TcpListener::bind(
        &format!("0.0.0.0:{}", CONFIG.backend_port))
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .context("error while booting server")?;

    Ok(())
}


// Function to handle timeout errors.
// It takes a boxed error as a parameter and returns a tuple with a status code and a JSON response.
async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "error":
                    format!(
                        "request took longer than the configured {} second timeout",
                        30
                    )
            })),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("unhandled internal error: {}", err) })),
        )
    }
}