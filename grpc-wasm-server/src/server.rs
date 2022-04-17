use tonic::{transport::Server, Request, Response, Status};

use hello::my_service_server::{MyService, MyServiceServer};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl MyService for MyGreeter {
    async fn hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello::HelloResponse {
            salute: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(MyServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
