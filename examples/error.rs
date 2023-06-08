use std::net::SocketAddr;
use std::{error, fmt};

use axum::Json;
use cabin::View;

async fn app() -> impl View {
    health().await
}

async fn health() -> Result<impl View, DbError> {
    test_database_connection().await?;
    Ok("Ok")
}

#[derive(Debug)]
struct DbError;

async fn test_database_connection() -> Result<(), DbError> {
    Err(DbError)
}

impl From<DbError> for cabin::Error {
    fn from(err: DbError) -> Self {
        cabin::Error::from_err(err)
    }
}

impl error::Error for DbError {}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error connecting to database")
    }
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|Json(event): Json<cabin::Event>| cabin::put_page(event, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
