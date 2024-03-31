use sea_orm::{ConnectionTrait, Statement};

// #[mry::mry]
pub async fn join<T: ConnectionTrait>(db: &T, v: i32) -> i32 {
    let ex: i32 = match db
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!("SELECT {v} AS ex"),
        ))
        .await
        .unwrap()
    {
        Some(qr) => qr.try_get("", "ex").unwrap(),
        None => 0,
    };

    dbg!(ex);

    ex + 10
}
