use axum::{Extension, Router, response::Html, routing::get};
use sea_orm::Iden;
use tera::{Context, Tera};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Auth " }))
}
