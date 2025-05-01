fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .build_client(false)
        .out_dir("src/api")
        .include_file("mod.rs")
        .build_client(false)
        .build_server(true)
        .compile_protos(
            &["../opentelemetry-proto/opentelemetry/proto/collector/trace/v1/trace_service.proto"],
            &["../opentelemetry-proto/"],
        )?;
    Ok(())
}
