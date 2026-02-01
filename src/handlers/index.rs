use crate::config::CONFIG;

use actix_web::{get, Responder};

/// Root Endpoint
///
/// Hello World Example
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        context_path = CONFIG.api_scope,
        path = "/",
        responses(
            (status = 200, description = "Hello World!")
        )
    )
)]
#[get("/")]
async fn service() -> impl Responder {
    "Hello World!".to_string()
}