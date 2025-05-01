fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto")?;

    tonic_build::configure()
        .build_client(false)
        .compile_protos(
            &["opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto"],
            &["opentelemetry-proto/"],
        )?;
    Ok(())
}
