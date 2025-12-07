use std::net::SocketAddr;
use std::{error, fmt};

use cabin::basic_document;
use cabin::prelude::*;
use cabin::view::boundary::Boundary;
use cabin::view::error::ErrorView;
use http::{Request, StatusCode};
use http_error::{AnyHttpError, HttpError};
use tokio::net::TcpListener;

async fn app(c: &Context) -> impl View {
    basic_document(c, boundary(c))
}

#[cabin::boundary(())]
fn boundary(c: &Context) -> Result<Boundary<()>, ExposedError> {
    if c.event::<()>().is_some() {
        return Err(ExposedError(
            http_error::error::bad_request("BOUNDARY ERROR").into(),
        ));
    }

    Ok(c.button().on_click(()).child("trigger error").boundary(()))
}

#[derive(Debug)]
pub struct ExposedError(AnyHttpError);

impl ErrorView for ExposedError {
    fn into_view(self, c: &Context) -> impl View {
        c.b().child(text!("{self}"))
    }

    fn should_render(&self) -> bool {
        let status_code = self.0.status_code();
        !status_code.is_redirection() && status_code != StatusCode::NO_CONTENT
    }
}

impl fmt::Display for ExposedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for ExposedError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source()
    }
}

impl<E> From<E> for ExposedError
where
    E: Into<AnyHttpError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl From<ExposedError> for Box<dyn HttpError + Send + 'static> {
    fn from(err: ExposedError) -> Self {
        err.0.into()
    }
}

cabin::BOUNDARIES!();

#[tokio::main]
async fn main() {
    let filter =
        tracing_subscriber::filter::filter_fn(|metadata| metadata.target().starts_with("cabin"));
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::new().pretty())
        .with(filter)
        .init();

    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer(&BOUNDARIES))
        .layer(cabin_service::livereload::layer())
        .layer(cabin_service::assets::layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
