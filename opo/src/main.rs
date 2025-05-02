mod api;

use api::opentelemetry::proto::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tonic::{Request, Response, Status, transport::Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct Service;

#[tonic::async_trait]
impl TraceService for Service {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        tracing::info!("Received request: {:#?}", request);
        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=trace,tower_http=debug,bollard=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let address = "[::1]:4317".parse()?;
    tracing::info!("Listening on {}", address);
    let service = Service;

    Server::builder()
        .add_service(TraceServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
