use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    // Its hard to trace errors while keeping multi thread logging.
    // Things can get really messy when multiple threads are logging at the same time.
    // One might pass while the other retunrs an error.
    // How will we know which thread is logging what?
    // We define random ids for each request, so we can trace them back to the request.
    let request_id = Uuid::new_v4();
    info!(
        "New subscription request received for email: {}, name: {}, request_id: {}",
        form.email, form.name, request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            info!(
                "New subscription created for email: {}, request_id: {}",
                form.email, request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // Notice that we are formatting error as a debug string, not a string.
            error!(
                "Failed to execute query: {:?}, request_id: {}",
                e, request_id
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
