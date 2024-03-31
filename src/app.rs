use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sea_orm::{Database, DatabaseConnection};

mod auth;

pub async fn main() {
    let db = Arc::new(Database::connect("sqlite::memory:").await.unwrap());

    let app = Router::new()
        .route("/", get(|| async {}))
        .nest("/auth", auth::router())
        .with_state(AppState { db });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    db: Arc<DatabaseConnection>,
}
