#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    arbitrary_self_types
)]
#![allow(incomplete_features)]

use std::net::SocketAddr;
use std::{error, fmt};

use axum::body::{Full, HttpBody};
use http::Response;
use rustend::component::{Component, PublicComponent};
use rustend::{rustend_scripts, rustend_stylesheets, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (
        rustend_stylesheets(),
        rustend_scripts(),
        Health::restore(()),
    )
}

#[derive(Debug, Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Health;

impl Component for Health {
    type Event = ();
    type Error = DbError;

    async fn update(&mut self, _: Self::Event) {}

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        test_database_connection().await?;
        Ok("Ok")
    }
}

#[derive(Debug)]
struct DbError;

async fn test_database_connection() -> Result<(), DbError> {
    Err(DbError)
}

impl From<DbError> for rustend::Error {
    fn from(err: DbError) -> Self {
        rustend::Error::from_err(err)
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
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
            }),
        )
        .layer(rustend_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
