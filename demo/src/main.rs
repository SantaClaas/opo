use axum::{
    Router,
    routing::{get, post},
};
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{ExportConfig, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    Resource,
    trace::{self, Sampler, SdkTracerProvider},
};
use opentelemetry_semantic_conventions::resource::{self};
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

    let service_data = Resource::builder()
        .with_attribute(KeyValue::new(
            resource::SERVICE_NAME,
            env!("CARGO_PKG_NAME"),
        ))
        .with_attribute(KeyValue::new(
            resource::SERVICE_VERSION,
            env!("CARGO_PKG_VERSION"),
        ))
        .build();

    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .with_batch_exporter(exporter)
        .with_resource(service_data)
        .build();

    let tracer = provider.tracer("axum_demo");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::from_default_env())
        // .with(tracing_subscriber::fmt::layer())
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env()
        //         .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        // )
        .with(telemetry_layer)
        .init();

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
        .layer(TraceLayer::new_for_http())
        .layer(axum_tracing_opentelemetry::middleware::OtelAxumLayer::default());

    // Run the app
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
