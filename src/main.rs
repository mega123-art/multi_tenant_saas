use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod cache;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;

use handlers::jobs::{create_job_handler, list_jobs_handler};
use handlers::projects::{
    create_project_handler, delete_project_handler, get_project_handler, list_projects_handler,
};
use handlers::tasks::{
    create_task_handler, get_subtasks_handler, list_tasks_handler, update_task_handler,
};
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

    //Initialize Redis
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");

    //Build router
    // PUBLIC routes (no middleware)
    let public_routes = Router::new()
        .route("/tenants", post(create_tenant_handler))
        .route("/tenants/:slug", get(get_tenant_handler));

    // PROTECTED routes (with tenant middleware)
    let protected_routes = Router::new()
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/:id", get(get_user_handler))
        .route(
            "/projects",
            post(create_project_handler).get(list_projects_handler),
        )
        .route(
            "/projects/:id",
            get(get_project_handler).delete(delete_project_handler),
        )
        .route("/tasks", post(create_task_handler).get(list_tasks_handler))
        .route("/tasks/:id", axum::routing::put(update_task_handler))
        .route("/tasks/:id/subtasks", get(get_subtasks_handler))
        .route("/jobs", post(create_job_handler).get(list_jobs_handler))
        .layer(axum::middleware::from_fn_with_state(
            (pool.clone(), redis_client.clone()),
            tenant_middleware,
        ));

    let app = public_routes
        .merge(protected_routes)
        .with_state((pool.clone(), redis_client.clone()))
        .layer(CorsLayer::permissive());

    //Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
