use axum::{
    Router,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    routing::get,
};
use std::sync::Arc;

mod home;

#[derive(Clone)]
pub struct AppState {
    lang_codes: Arc<Vec<String>>,
}

async fn language_redirector(
    Path(tail): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut segments = tail.splitn(2, '/');

    let lang = segments.next().unwrap_or("");
    let rest = segments.next().unwrap_or("");

    if state.lang_codes.contains(&lang.to_string()) {
        Redirect::to(&format!("/{}/{}", lang, rest))
    } else {
        let redirect_path = if tail.is_empty() {
            "/en/".to_string()
        } else {
            format!("/en/{}", tail)
        };
        Redirect::to(&redirect_path)
    }
}

pub fn router(lang_codes: Vec<String>) -> Router {
    let state = AppState {
        lang_codes: Arc::new(lang_codes),
    };

    Router::new()
        .nest("/{lang}", home::router())
        .route("/{*tail}", get(language_redirector))
        .with_state(state)
}
