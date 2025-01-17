use std::net::SocketAddr;
use std::time::{Duration, Instant};

use cabin::basic_document;
use cabin::prelude::*;
use cabin::view::FutureExt;
use http::Request;
use tokio::net::TcpListener;

async fn app() -> impl View {
    let start = Instant::now();
    basic_document(h::ul((
        delayed(start, Duration::from_secs(1)).into_view(),
        delayed(start, Duration::from_secs(2)).into_view(),
        delayed(start, Duration::from_secs(3)).into_view(),
        h::text!("page finished after {:.2}", start.elapsed().as_secs_f64()),
    )))
}

async fn delayed(start: Instant, delay: Duration) -> impl View {
    let started_at = start.elapsed();
    let inner = Instant::now();
    tokio::time::sleep(delay).await;
    let task_local = TASK_LOCAL
        .try_with(|_| "task local works")
        .unwrap_or("task local DOES NOT work");
    h::li(h::text!(
        "delayed for {:?}, started after {:.2}, took {:.2}, finished after {:.2} -- {task_local}",
        delay,
        started_at.as_secs_f64(),
        inner.elapsed().as_secs_f64(),
        start.elapsed().as_secs_f64(),
    ))
}

tokio::task_local! {
    static TASK_LOCAL: &'static str;
}

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
            axum::routing::get(|| TASK_LOCAL.scope("task local works", cabin::get_page(app))).put(
                |req: Request<axum::body::Body>| {
                    cabin::put_page(req, || TASK_LOCAL.scope("task local works", app()))
                },
            ),
        )
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer())
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
