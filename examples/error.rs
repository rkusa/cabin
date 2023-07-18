use std::net::SocketAddr;
use std::{error, fmt};

use cabin::View;
use http::{Request, StatusCode};
use http_error::AnyHttpError;

async fn app() -> impl View {
    health().await
}

async fn health() -> Result<impl View, AnyHttpError> {
    test_database_connection().await?;
    Ok("Ok")
}

#[derive(Debug)]
struct DbError;

async fn test_database_connection() -> Result<(), DbError> {
    Err(DbError)
}

impl error::Error for DbError {}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error connecting to database")
    }
}

impl http_error::HttpError for DbError {
    fn status_code(&self) -> http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
