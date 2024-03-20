use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{query, types::chrono::Utc, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(pool: web::Data<PgPool>, form: web::Form<FormData>) -> impl Responder {
    match query!(
        "
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        ",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            eprintln!("Failed to execute query: {err}");
            HttpResponse::InternalServerError()
        }
    }
}
