use super::{super::AppError, service};

pub async fn join() -> Result<String, AppError> {
    Ok(service::join(true).to_string())
}

#[cfg(test)]
mod tests {
    use super::{service, *};

    #[tokio::test]
    #[mry::lock(service::join)]
    async fn join_calls_service_join() {
        let mock_join = service::mock_join(true).returns(10);

        assert_eq!(join().await.unwrap(), "10");

        mock_join.assert_called(1);
    }
}
