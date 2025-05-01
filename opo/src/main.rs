mod api;

use api::opentelemetry::proto::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
    trace_service_server::{TraceService, TraceServiceServer},
};
use tonic::{Request, Response, Status, transport::Server};

struct Service;

#[tonic::async_trait]
impl TraceService for Service {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        println!("Received request: {:?}", request);
        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:4317".parse()?;
    let service = Service;

    Server::builder()
        .add_service(TraceServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
