use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::style::preset::{BLACK, SM};
use rustend::style::text;
use rustend::{css, html, View};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(app);
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{addr}");
    server.await.unwrap();
}

async fn app() -> impl View {
    counter(0).await
}

#[rustend::component]
async fn counter(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    html::button(html::text!("{count}"))
        .on_click(incr, ())
        // TODO: prevent dynamic values
        // TODO: conditional styles
        // TODO: auto-complete
        .class(css!(text(BLACK), text(SM)))
}
