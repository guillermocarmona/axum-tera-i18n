use axum::{Extension, Router};
//use std::env;
//use std::time::Duration;

use tera::Tera;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use fluent_templates::{FluentLoader, Loader, static_loader};

mod api;
mod pages;

static_loader! {
    static LOCALES = {
        locales: "src/locales",
        fallback_language: "en"
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenvy::dotenv().expect("Failed to load .env file");

    let mut tera = Tera::new("src/templates/**/*.html").expect("Failed to load templates");

    println!("Templates:");
    for name in tera.get_template_names() {
        println!(" - {}", name);
    }

    tera.register_function("t", FluentLoader::new(&*LOCALES));

    let lang_codes: Vec<String> = LOCALES.locales().map(|langid| langid.to_string()).collect();

    let app = Router::new()
        .merge(pages::router(lang_codes))
        .nest("/api", api::router())
        //.layer(Extension(db))
        .layer(Extension(tera))
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
