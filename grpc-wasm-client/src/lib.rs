use hello::my_service_client::MyService;
use hello::HelloRequest;
extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;

pub mod hello {
    include!(concat!(env!("OUT_DIR"), concat!("/hello.rs")));
    //tonic::include_proto!("hello");
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

#[wasm_bindgen]
pub async fn hi() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // this is the address of envoy or any other proxy that will forward the request to the real grpc service
    // https://grpc.io/docs/platforms/web/basics/
    let mut client = MyService::new("http://localhost:8080".into());
    let request = HelloRequest {
        name: "Tonic".into(),
    };

    let response = client.hello(request).await.unwrap();

    alert(&format!("RESPONSE={:?}", response));
}
