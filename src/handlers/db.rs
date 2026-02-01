use crate::db::DbPool;
use crate::config::CONFIG;
use sqlx::PgPool;

use actix_web::{get, web, HttpResponse, Responder};

/// Root Endpoint
///
/// Hello World Example
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        context_path = CONFIG.api_scope,
        path = "/db",
        responses(
            (status = 200, description = "Hello World!")
        )
    )
)]
#[get("/db")]
async fn check_dba(pool: web::Data<DbPool>) -> impl Responder {
    match check_db(pool.get_ref()).await {
        Ok(v) => HttpResponse::Ok().body(format!("DB OK: {}", v)),
        Err(e) => {
            log::error!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn check_db(pool: &PgPool) -> sqlx::Result<i32> {
    let (v,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;

    Ok(v)
}
