use axum::extract::{Query, State};

use super::{
    super::{AppError, AppState},
    data::AuthRequestData,
    service,
};

pub async fn join(
    state: State<AppState>,
    data: Query<AuthRequestData>,
) -> Result<String, AppError> {
    Ok(service::join(&state.db, &data, true).await.to_string())
}

// #[cfg(test)]
// mod tests {
//     use super::{service, *};

//     #[tokio::test]
//     #[mry::lock(service::join)]
//     async fn join_calls_service_join() {
//         let mock_join = service::mock_join(true).returns(10);

//         assert_eq!(join().await.unwrap(), "10");

//         mock_join.assert_called(1);
//     }
// }
