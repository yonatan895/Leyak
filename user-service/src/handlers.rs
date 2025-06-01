use crate::models::{NewUser, User};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn register_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (github_id, username) VALUES ($1, $2) RETURNING id, github_id, username",
    )
    .bind(&new_user.github_id)
    .bind(&new_user.username)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, github_id, username FROM users WHERE id = $1",
    )
    .bind(*user_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
