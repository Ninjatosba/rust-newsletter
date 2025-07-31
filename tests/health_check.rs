use actix_web::{HttpResponse, ResponseError, dev::Response};
use newsletter::run;

#[actix_rt::test]
async fn health_check_works() {
    let _ = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health")
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> std::io::Result<()> {
    let server = run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
    Ok(())
}
