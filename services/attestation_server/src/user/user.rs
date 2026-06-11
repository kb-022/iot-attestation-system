use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User{
    pub(crate) device_id: String,
    pub(crate) public_key: Vec<u8>,
    pub(crate) curve: String,
}


//Tutorial followed for get/post: https://youtu.be/cJyl9e2oqHY?si=LJLUoAkB8-VKpeLm
impl User{
    pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE device_id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn add(State(pool): State<PgPool>, Json(user): Json<User>) -> Result<(StatusCode, Json<User>), StatusCode>{
        let existing_user = sqlx::query!("SELECT * FROM users WHERE device_id = $1",user.device_id)
            .fetch_optional(&pool)
            .await
            .unwrap();

        if existing_user.is_some(){
            return Err(StatusCode::CONFLICT);
        }

        sqlx::query_as::<_, User>("INSERT INTO users (device_id, public_key, curve) VALUES ($1, $2, $3) RETURNING *")
            .bind(user.device_id)
            .bind(user.public_key)
            .bind(user.curve)
            .fetch_one(&pool)
            .await
            .map(|u| (StatusCode::CREATED, Json(u)))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}