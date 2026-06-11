use crate::cryptography::crypto;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
#[derive(Serialize,Deserialize, Debug)]
pub struct Response{
    pub(crate) device_id: String,
    pub(crate) data: Vec<u8>,
    pub(crate) signature: Vec<u8>,
}

pub async fn get_challenge() -> Json<crypto::Challenge>{
    Json(crypto::generate_challenge())
}

pub async fn post_response(State(pool): State<PgPool>,Json(r): Json<Response>) -> Result<Json<bool>,StatusCode>{
    let device_id = r.device_id.clone();
    let verification_result = crypto::verify_response(device_id,r,&pool).await;
    match verification_result {
        Ok(result) => {
            println!("Verification successful!");
            Ok(Json(result))
        }
        Err(_) => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}