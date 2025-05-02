use axum::{
    Router,
    routing::{get, post},
};
use opentelemetry::global;
use opentelemetry_otlp::{ExportConfig, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::trace::{self, Sampler, SdkTracerProvider};
use std::{collections::HashMap, iter::Map, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::{error, span};
use tracing_subscriber::{Registry, layer::SubscriberExt, util::SubscriberInitExt};

use opentelemetry::trace::TracerProvider as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .with_simple_exporter(exporter)
        .build();

    let tracer = provider.tracer("readme_example");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });

    // Build the Axum app
    let app = Router::new()
        .route(
            "/",
            get(async || {
                tracing::info!("Received hello request");
                "Welcome to the Axum app!"
            }),
        )
        .route("/work", post(|| async { "Work done!" }))
        .layer(TraceLayer::new_for_http());

    // Run the app
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;
    // Shutdown OpenTelemetry
    // global::shutdown_tracer_provider();

    Ok(())
}
