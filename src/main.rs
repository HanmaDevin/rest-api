mod handlers;
mod models;
mod utils;

use std::env;

use crate::handlers::*;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let db_url = env::var("DB_URL").expect("Database url not set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Could not connect to Database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the User Management API!"
}
