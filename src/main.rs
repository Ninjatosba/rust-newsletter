use newsletter::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        get_configuration().unwrap_or_else(|e| panic!("Failed to read configuration: {}", e));
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    run(listener)?.await
}
