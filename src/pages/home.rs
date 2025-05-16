use axum::{Extension, Router, extract::State, response::Html, routing::get};
use tera::{Context, Tera};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}

async fn index(
    Extension(tera): Extension<Tera>,
    State(app_state): State<AppState>,
) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("lang", &app_state.lang);

    let template = tera
        .render("home/index.html", &ctx)
        .expect("Error rendering template");

    Html(template)
}
