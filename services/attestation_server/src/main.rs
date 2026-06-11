mod cryptography;
mod user;
mod routes;

use crate::cryptography::crypto;
use crate::routes::auth_routes;
use crate::routes::user_routes;
use axum::routing::{get, post};
use axum::Router;
use dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;


//Initial Axum setup from: https://youtu.be/7RlVM0D4CEA?si=At4fNmawXFqsoqdT
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Database connection failed");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    let _ = crypto::construct_verifying_key("kb-test".to_string(), &pool).await;

    let app = Router::new()
        .route("/",get(root))
        .route("/enroll",post(user_routes::user_add))
        .route("/users",get(user_routes::user_get_all))
        .route("/user/{id}",get(user_routes::user_get))
        .route("/auth/challenge",get(auth_routes::get_challenge))
        .route("/auth/response",post(auth_routes::post_response))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener,app)
        .await
        .expect("Failed to run server");
}

async fn root() -> &'static str {
    "FYP Verifier API"
}