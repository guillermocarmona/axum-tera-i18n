use axum::{Router,  response::Redirect, routing::get};

mod home;
pub fn router(lang_codes: Vec<String>) -> Router {
    let mut router = Router::new()
       .route("/", get(|| async { Redirect::to("/en") }));
 
    for lang in lang_codes {
        router = router.nest(&format!("/{}", lang), home::router());
    }

    router.route("/{*_}", get(|| async {
        Redirect::to("/en")
    }))
}

