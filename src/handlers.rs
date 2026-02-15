use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::{
    models::{User, UserPayload},
    utils::valid_email,
};

pub async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    if valid_email(payload.get_email()) {
        sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
            .bind(payload.get_name())
            .bind(payload.get_email())
            .fetch_one(&pool)
            .await
            .map(|u| (StatusCode::CREATED, Json(u)))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
        .bind(payload.get_name())
        .bind(payload.get_email())
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}
