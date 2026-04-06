use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod models;
mod errors;
mod db;
mod handlers;
mod middleware;

use handlers::users::{
    create_user_handler,
    list_users_handler,
    get_user_handler,
};

use middleware::tenant::tenant_middleware;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    //Create DB pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    //Build router
    let app = Router::new()
        // user routes
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/:id", get(get_user_handler))
        // apply tenant middleware ONLY to these routes
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        // shared state
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());

    //Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}