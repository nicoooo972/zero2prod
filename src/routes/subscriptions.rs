use actix_web::{web, HttpResponse};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<sqlx::PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(connection.get_ref())
        .await
        .expect("TODO: panic message");
    HttpResponse::Ok().finish()
}