use api_common::{cors_response, get_db_pool, handle_options};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vercel_runtime::{run, Error, Request, Response, StatusCode};

#[derive(Deserialize, Debug)]
pub struct SaveProjectRequest {
    pub id: Option<String>,
    pub user_id: Uuid,
    pub title: String,
    pub timeline_json: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct SaveProjectResponse {
    pub success: bool,
    pub id: String,
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<String>, Error> {
    if req.method() == http::Method::OPTIONS {
        return handle_options();
    }

    if req.method() != http::Method::POST {
        return cors_response(
            StatusCode::METHOD_NOT_ALLOWED,
            serde_json::json!({ "error": "Method not allowed. Use POST." }).to_string(),
        );
    }

    let body_bytes = req.body();
    let payload: SaveProjectRequest = match serde_json::from_slice(body_bytes) {
        Ok(p) => p,
        Err(e) => {
            return cors_response(
                StatusCode::BAD_REQUEST,
                serde_json::json!({ "error": format!("Invalid JSON payload: {}", e) }).to_string(),
            );
        }
    };

    let pool = match get_db_pool().await {
        Ok(p) => p,
        Err(e) => {
            return cors_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({ "error": format!("Database connection error: {}", e) }).to_string(),
            );
        }
    };

    let project_id = match payload.id {
        Some(id_str) => Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4()),
        None => Uuid::new_v4(),
    };

    let query_result = sqlx::query!(
        r#"
        insert into projects (id, user_id, title, timeline_json, updated_at)
        values ($1, $2, $3, $4, now())
        on conflict (id) do update
        set title = excluded.title,
            timeline_json = excluded.timeline_json,
            updated_at = now()
        "#,
        project_id,
        payload.user_id,
        payload.title,
        payload.timeline_json
    )
    .execute(&pool)
    .await;

    match query_result {
        Ok(_) => {
            let resp = SaveProjectResponse {
                success: true,
                id: project_id.to_string(),
                message: "Project timeline saved successfully.".to_string(),
            };
            cors_response(StatusCode::OK, serde_json::to_string(&resp).unwrap())
        }
        Err(e) => cors_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::json!({ "error": format!("Database query failed: {}", e) }).to_string(),
        ),
    }
}
