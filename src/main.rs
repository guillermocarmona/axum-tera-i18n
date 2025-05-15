use axum::{Extension, Router};
//use sea_orm::{ConnectOptions, Database};
//use std::env;
//use std::time::Duration;

use tera::Tera;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use fluent_templates::{FluentLoader, static_loader};

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

    // let db_connection = env::var("DATABASE_URL")?;

    // let mut opt = ConnectOptions::new(&db_connection);
    // opt.max_connections(10)
    //    .connect_timeout(Duration::from_secs(5))
    //    .sqlx_logging(true);

    // let db = Database::connect(opt).await?;

    let mut tera = Tera::new("src/templates/**/*.html").expect("Failed to load templates");

    println!("Templates:");
    for name in tera.get_template_names() {
        println!(" - {}", name);
    }

    tera.register_function("t", FluentLoader::new(&*LOCALES));

    let app = Router::new()
        .merge(pages::router())
        .nest("/api", api::router())
        //.layer(Extension(db))
        .layer(Extension(tera))
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
