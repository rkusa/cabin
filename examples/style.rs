use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::{html, View};
use rustend_css::{self as css, css, Style};

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
        .class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            css!(
                css::BLOCK,
                css::text::BLACK,
                css::text::SM,
                css::bg::BLACK.hover(),
                css::text::WHITE.hover(),
                css::text::XS.hover().focus(),
            ) + (count == 0).then_some(css!(css::text::color("red"))),
        )
}
