pub mod graphql;
mod misc;

pub mod ws {
    use crate::misc::ExtractUserAgent;
    use axum::ws::{Message, WebSocket};

    pub async fn sub_(mut socket: WebSocket, ExtractUserAgent(user_agent): ExtractUserAgent) {
        println!("`{:?}` connected", user_agent);

        if let Some(msg) = socket.recv().await {
            let msg = msg.unwrap();
            println!("Client says: {:?}", msg);
            if msg.is_text() {}
            if msg.is_ping() {
                socket.send(Message::pong(msg.as_bytes())).await.unwrap();
            } else {
                socket.send(msg).await.unwrap();
            }

            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            socket.send(Message::text("Bye.")).await.unwrap();
        }
    }
}

pub mod db {
    pub use axum::extract::Extension;
    use http::StatusCode;

    //type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
    pub type PoolPg = sqlx::Pool<sqlx::postgres::Postgres>;

    pub async fn pub_(Extension(pool): Extension<PoolPg>) -> Result<String, (StatusCode, String)> {
        // We cannot get a connection directly via an extractor because
        // `bb8::PooledConnection` contains a reference to the pool and
        // `extract::FromRequest` cannot return types that contain references.
        //
        // So therefore we have to get a connection from the pool manually.
        //let conn = pool.get().await.map_err(internal_error)?;
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await
            .map_err(internal_error)?;
        assert_eq!(row.0, 150);
        println!("SELECT $1 .bind {}", row.0);

        //let row = conn.query_one("select 1 + 1", &[]) .await .map_err(internal_error)?;
        //let two: i32 = row.try_get(0).map_err(internal_error)?;

        Ok(row.0.to_string())
    }

    /// Utility function for mapping any error into a `500 Internal Server Error`
    /// response.
    fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}
