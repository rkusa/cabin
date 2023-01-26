use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::previous::previous;
use rustend::{html, view, IntoView, View};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(|| app(true));
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}

#[rustend::component]
async fn app(enabled: bool) -> impl View {
    async fn toggle(enabled: bool, _: ()) -> bool {
        !enabled
    }

    // Just a way to test a rerender without actually chaning anything
    async fn reset(enabled: bool, _: ()) -> bool {
        enabled
    }

    view![
        html::div(if enabled {
            view![
                html::div![counter(0), " (state reset on parent rerender)"],
                html::div![
                    counter(previous(|c| c)),
                    " (state restored on parent rerender)"
                ],
            ]
            .boxed()
        } else {
            "...".boxed()
        }),
        view![
            html::button(if enabled {
                "remove counter"
            } else {
                "add counter"
            })
            .on_click(toggle, ()),
            html::button("force rerender").on_click(reset, ()),
        ]
    ]
}

#[rustend::component]
async fn counter(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    html::button(html::text!("👍 {}", count)).on_click(incr, ())
}
