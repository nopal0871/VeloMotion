use sqlx::{PgPool, Pool, Postgres};
use vercel_runtime::{Response, StatusCode};

pub async fn get_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    PgPool::connect(&database_url).await
}

pub fn cors_response(status: StatusCode, body: String) -> Result<Response<String>, vercel_runtime::Error> {
    Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| e.into())
}

pub fn handle_options() -> Result<Response<String>, vercel_runtime::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .body("".to_string())
        .map_err(|e| e.into())
}
