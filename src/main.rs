use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod db;
mod errors;
mod handlers;
mod middleware;
mod models;

use handlers::tenants::{create_tenant_handler, get_tenant_handler};
use handlers::users::{create_user_handler, get_user_handler, list_users_handler};

use middleware::tenant::tenant_middleware;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    //Create DB pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    //Build router
    // PUBLIC routes (no middleware)
    let public_routes = Router::new()
        .route("/tenants", post(create_tenant_handler))
        .route("/tenants/:slug", get(get_tenant_handler));

    // PROTECTED routes (with tenant middleware)
    let protected_routes = Router::new()
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/:id", get(get_user_handler))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ));
        

    let app = public_routes
    .merge(protected_routes)
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
