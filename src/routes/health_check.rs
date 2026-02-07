use actix_web::{HttpRequest, HttpResponse, Responder};
pub async fn health_check(_req: HttpRequest) -> impl Responder {
    println!("health check endpoint hit");
    HttpResponse::Ok().finish()
}
