#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use axum::body::{Full, HttpBody};
use axum::response::Response;
use cabin::component::{Component, PublicComponent};
use cabin::view::FutureExt;
use cabin::{cabin_scripts, cabin_stylesheets, html, View};

async fn app() -> impl View {
    (cabin_stylesheets(), cabin_scripts(), root().await)
}

async fn root() -> impl View {
    let start = Instant::now();
    html::ul((
        delayed(start, Duration::from_secs(1)).into_view(),
        delayed(start, Duration::from_secs(2)).into_view(),
        delayed(start, Duration::from_secs(3)).into_view(),
        Delayed::restore_or_else((), || Delayed::new(start, Duration::from_secs(4))),
        html::text!("page finished after {:.2}", start.elapsed().as_secs_f64()),
    ))
}

async fn delayed(start: Instant, delay: Duration) -> impl View {
    let started_at = start.elapsed();
    let inner = Instant::now();
    tokio::time::sleep(delay).await;
    html::li(html::text!(
        "delayed for {:?}, started after {:.2}, took {:.2}, finished after {:.2}",
        delay,
        started_at.as_secs_f64(),
        inner.elapsed().as_secs_f64(),
        start.elapsed().as_secs_f64(),
    ))
}

#[derive(Hash, serde::Serialize, serde::Deserialize, PublicComponent)]
struct Delayed {
    #[serde(skip)]
    inner: DelayedInner,
}

#[derive(Hash)]
struct DelayedInner {
    start: Instant,
    delay: Duration,
}

impl Default for DelayedInner {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            delay: Duration::from_secs(1),
        }
    }
}

impl Delayed {
    fn new(start: Instant, delay: Duration) -> Self {
        Self {
            inner: DelayedInner { start, delay },
        }
    }
}

impl Component for Delayed {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _event: Self::Event) {}

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        let started_at = self.inner.start.elapsed();
        let inner = Instant::now();
        tokio::time::sleep(self.inner.delay).await;
        Ok(html::li(html::text!(
            "delayed for {:?}, started after {:.2}, took {:.2}, finished after {:.2}",
            self.inner.delay,
            started_at.as_secs_f64(),
            inner.elapsed().as_secs_f64(),
            self.inner.start.elapsed().as_secs_f64(),
        )))
    }
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = cabin::render_to_response(app).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
            }),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
