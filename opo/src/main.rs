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
        // tracing::info!("Received request: {:#?}", request);
        let data = request.get_ref();
        tracing::info!("Received {} spans", &data.resource_spans.len());
        // tracing::info!("First span: {:#?}", &data.resource_spans.get(0));
        for span in &data.resource_spans {
            // tracing::info!("Span: {:#?}", span.resource);

            let Some(resource) = &span.resource else {
                continue;
            };

            let attributes: Vec<String> = resource
                .attributes
                .iter()
                .map(|attribute| attribute.key.clone())
                .collect();

            for attribute in attributes {
                if !attribute.starts_with("http") {
                    continue;
                }

                tracing::info!("Attribute: {:?}", attribute);
            }
        }

        data.resource_spans
            .iter()
            .flat_map(|span| span.scope_spans.clone())
            .flat_map(|scope_span| scope_span.spans)
            .flat_map(|span| span.attributes.clone())
            .for_each(|attribute| {
                //
                if !attribute.key.starts_with("http") {
                    return;
                }
                tracing::info!("Attribute 2: {:?}", attribute.key);
            });

        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=trace,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
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
