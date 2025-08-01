use std::net::TcpListener;

use newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    run(listener)?.await
}
