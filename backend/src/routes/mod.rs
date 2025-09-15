use axum::{Router,extract::Query,routing::get,Json, response::IntoResponse};
use std::collections::HashMap;
use crate::models::{SearchParams, Company};
use crate::error::AppError;
use std::env;
use serde_json::Value;

pub fn app_routes() -> Router {
    Router::new()
        .route("/test", get(|| async {"API is live." }))
}
