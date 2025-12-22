use crate::AppState;
use crate::authentication::login;
use crate::authentication::register;
use axum::{Router, routing::{post,get}};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", post(register::register))
        .with_state(state)
}
