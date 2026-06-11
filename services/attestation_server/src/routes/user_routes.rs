use crate::user::user::User;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

pub async fn user_get(State(pool): State<PgPool>, Path(id): Path<String>) -> Result<Json<User>, StatusCode> {
    User::get_by_id(&pool, &id).await
        .map(|user| Json(user))
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn user_get_all(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    User::get_all(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn user_add(State(pool): State<PgPool>,Json(user): Json<User>) -> Result<(StatusCode,Json<User>), StatusCode>{
    User::add(State(pool), Json(user)).await
}
