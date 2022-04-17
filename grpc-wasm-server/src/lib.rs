use hello::HelloRequest;

use wasm_bindgen::prelude::*;

pub mod hello {
    tonic::include_proto!("hello");
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/*
async fn hi() -> String {
    let wasm_conn = Client::connect("ws://localhost:50051").await.unwrap();
    let mut client = MyServiceClient::new(wasm_conn);
    //let mut client = MyServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.hello(request).await.unwrap();

    format!("RESPONSE={:?}", response)
}*/
