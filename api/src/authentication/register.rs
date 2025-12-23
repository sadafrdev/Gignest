use crate::AppState;
use crate::lib::enums::{Country, Role};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, FromRow, Validate)]
pub struct Users {
    pub first_name: String,
    pub last_name: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub phone_number: String,
    pub username: String,
    pub country: Country,
    pub role: Role,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Users>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
        INSERT INTO users 
        (first_name, last_name, password, email, phone_number, username, country, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ",
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.password)
    .bind(payload.email)
    .bind(payload.phone_number)
    .bind(payload.username)
    .bind(payload.country as Country)
    .bind(payload.role as Role)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
