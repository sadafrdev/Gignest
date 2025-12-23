use crate::AppState;
use crate::authentication::login;
use crate::authentication::register;
use crate::authentication::forgot_password::request_password_reset;
use axum::{Router, routing::{post,get}};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", post(register::register))
        .route("/forgot_password", get(request_password_reset))
        .with_state(state)
}
