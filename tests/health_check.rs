use std::net::TcpListener;

use actix_web::{HttpResponse, ResponseError, dev::Response};
use newsletter::run;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}", address.to_string())
}
