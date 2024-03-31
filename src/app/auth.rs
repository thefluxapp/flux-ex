use axum::{routing::get, Router};

use super::AppState;

mod controller;
mod data;
mod repo;
mod service;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(controller::join))
}
