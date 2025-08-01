use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", name)
}
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
