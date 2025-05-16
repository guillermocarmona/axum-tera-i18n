use axum::{Router, http::Uri, response::Redirect, routing::get};

mod auth;
mod home;

pub fn router(lang_codes: Vec<String>) -> Router {
    let mut router = Router::new().route("/", get(|| async { Redirect::to("/en") }));

    for lang in lang_codes {
        let new_router = Router::new()
            .merge(home::router())
            .nest("/auth", auth::router());

        router = router.nest(&format!("/{}", lang), new_router);
    }

    router.route("/{*_}", get(|| async { Redirect::to("/en") }))
}

pub fn get_lang(uri: Uri) -> String {
    uri.path()
        .trim_start_matches('/')
        .splitn(2, '/')
        .next()
        .unwrap_or("en")
        .to_string()
}
