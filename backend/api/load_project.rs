use api_common::{cors_response, get_db_pool, handle_options};
use serde::Serialize;
use uuid::Uuid;
use vercel_runtime::{run, Error, Request, Response, StatusCode};

#[derive(Serialize, Debug)]
pub struct ProjectRecord {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub timeline_json: serde_json::Value,
    pub updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<String>, Error> {
    if req.method() == http::Method::OPTIONS {
        return handle_options();
    }

    if req.method() != http::Method::GET {
        return cors_response(
            StatusCode::METHOD_NOT_ALLOWED,
            serde_json::json!({ "error": "Method not allowed. Use GET." }).to_string(),
        );
    }

    let query_string = req.uri().query().unwrap_or("");
    let params: std::collections::HashMap<String, String> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();

    let project_id_str = match params.get("id") {
        Some(id) => id,
        None => {
            return cors_response(
                StatusCode::BAD_REQUEST,
                serde_json::json!({ "error": "Missing query parameter 'id'." }).to_string(),
            );
        }
    };

    let project_uuid = match Uuid::parse_str(project_id_str) {
        Ok(u) => u,
        Err(_) => {
            return cors_response(
                StatusCode::BAD_REQUEST,
                serde_json::json!({ "error": "Invalid project UUID format." }).to_string(),
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

    let rec = sqlx::query!(
        r#"
        select id, user_id, title, timeline_json, updated_at
        from projects
        where id = $1
        "#,
        project_uuid
    )
    .fetch_optional(&pool)
    .await;

    match rec {
        Ok(Some(row)) => {
            let project = ProjectRecord {
                id: row.id.to_string(),
                user_id: row.user_id.to_string(),
                title: row.title,
                timeline_json: row.timeline_json,
                updated_at: row.updated_at.to_string(),
            };
            cors_response(StatusCode::OK, serde_json::to_string(&project).unwrap())
        }
        Ok(None) => cors_response(
            StatusCode::NOT_FOUND,
            serde_json::json!({ "error": "Project not found." }).to_string(),
        ),
        Err(e) => cors_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::json!({ "error": format!("Database query failed: {}", e) }).to_string(),
        ),
    }
}
