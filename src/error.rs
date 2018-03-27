use std::result;
use std::error;
use rocket;
use rocket_contrib::Json;

pub fn error_handlers() -> Vec<rocket::Catcher> {
    errors![not_found, conflict, internal_server_error]
}

pub type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug)]
pub enum Error {}

// Rocket Error Handler Section
#[error(404)]
fn not_found() -> Json {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[error(409)]
fn conflict() -> Json {
    Json(json!({
        "status": "conflict",
        "reason": "Duplicate resource found."
    }))
}

#[error(500)]
fn internal_server_error() -> Json {
    Json(json!({
        "status": "error",
        "reason": "An internal server error."
    }))
}
