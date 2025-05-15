use axum::Router;
mod home;

pub fn router() -> Router {
    Router::new().merge(home::router())
}
