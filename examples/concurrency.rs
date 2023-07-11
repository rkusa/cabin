use std::net::SocketAddr;
use std::time::{Duration, Instant};

use axum::Json;
use cabin::view::FutureExt;
use cabin::{html, View};

async fn app() -> impl View {
    let start = Instant::now();
    html::ul(
        (),
        (
            delayed(start, Duration::from_secs(1)).into_view(),
            delayed(start, Duration::from_secs(2)).into_view(),
            delayed(start, Duration::from_secs(3)).into_view(),
            html::text!("page finished after {:.2}", start.elapsed().as_secs_f64()),
        ),
    )
}

async fn delayed(start: Instant, delay: Duration) -> impl View {
    let started_at = start.elapsed();
    let inner = Instant::now();
    tokio::time::sleep(delay).await;
    html::li(
        (),
        html::text!(
            "delayed for {:?}, started after {:.2}, took {:.2}, finished after {:.2}",
            delay,
            started_at.as_secs_f64(),
            inner.elapsed().as_secs_f64(),
            start.elapsed().as_secs_f64(),
        ),
    )
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
