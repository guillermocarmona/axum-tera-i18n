use axum::{Extension, Router, extract::OriginalUri, http::Uri, response::Html, routing::get};
use tera::{Context, Tera};

use super::get_lang;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index(Extension(tera): Extension<Tera>, OriginalUri(uri): OriginalUri) -> Html<String> {
    let mut ctx = Context::new();

    let lang = get_lang(uri);
    ctx.insert("lang", &lang);

    let template = tera
        .render("home/index.html", &ctx)
        .expect("Error rendering template");

    Html(template)
}
