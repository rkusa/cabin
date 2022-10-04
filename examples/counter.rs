use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::{html, IntoView, View};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(app);
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}

async fn app() -> impl View {
    counter(0).await
}

#[rustend::component]
async fn counter(count: u32) -> impl View<u32> {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    (
        //     (self.0 == 0).then(|| ),
        //     (self.0 > 0).then(move || html::div(html::text!("Count: {}", self.0))),
        if count > 0 {
            // TODO: reintroduce html::text!
            html::div(format!("Count: {}", count)).boxed()
        } else {
            html::div("Hit `incr` to start counting ...").boxed()
        },
        html::button("incr").on_click(incr, ()),
    )
}
