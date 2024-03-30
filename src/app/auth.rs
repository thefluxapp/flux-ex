use axum::{routing::get, Router};

mod controller;
mod repo;
mod service;

pub fn router() -> Router {
    Router::new().route("/", get(controller::join))
}
