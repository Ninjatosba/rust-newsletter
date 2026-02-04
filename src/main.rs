use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        get_configuration().unwrap_or_else(|e| panic!("Failed to read configuration: {}", e));
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(address).expect("Failed to bind port 8080");
    run(listener, connection_pool)?.await
}
