fn main() -> Result<(), Box<dyn std::error::Error>> {
    rust_grpc_web::configure()
        .support_streaming(false)
        .compile(&["../proto/hello.proto"], &["../proto/"])?;
    Ok(())
}
