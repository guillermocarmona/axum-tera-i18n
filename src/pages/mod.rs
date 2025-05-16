use axum::{Router, response::Redirect, routing::get};
mod auth;
mod home;

#[derive(Clone)]
struct AppState {
    lang: String,
}

pub fn router(lang_codes: Vec<String>) -> Router<AppState> {
    let mut router = Router::new()
        .route("/", get(|| async { Redirect::to("/en") }))
        .with_state(AppState {
            lang: "en".to_string(),
        });

    for lang in lang_codes {
        let state = AppState { lang: lang.clone() };
        let new_router = Router::new()
            .merge(home::router())
            .nest("/auth", auth::router())
            .with_state(state);

        router = router.nest(&format!("/{}", lang), new_router);
    }

    router.route("/{*_}", get(|| async { Redirect::to("/en") }))
}
