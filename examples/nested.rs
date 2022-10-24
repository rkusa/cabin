use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::{html, View};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(|| app(()));
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}

#[rustend::component]
async fn app(_state: ()) -> impl View {
    async fn reset(_state: (), _: ()) {}

    (
        html::div(counter(0).await),
        html::button("reset").on_click(reset, ()),
    )
}

#[rustend::component]
async fn counter(count: u32) -> impl View<u32> {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    html::button(html::text!("👍 {}", count)).on_click(incr, ())
}
