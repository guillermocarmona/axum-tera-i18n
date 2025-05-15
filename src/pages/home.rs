use axum::{Extension, Router, response::Html, routing::get};
use tera::{Context, Tera};

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index(Extension(tera): Extension<Tera>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "");

    let template = tera
        .render("home/index.html", &context)
        .expect("Error rendering template");

    Html(template)
}
