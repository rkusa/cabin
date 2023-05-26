use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::view::fragment;
use rustend::{html, rustend_scripts, rustend_stylesheets, View};

async fn app() -> impl View {
    fragment() >> rustend_stylesheets() >> rustend_scripts() >> counter(0).await
}

#[rustend::component]
async fn counter(count: u32) -> Result<impl View, Infallible> {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    Ok(fragment()
        // >> (count > 0).then(|| html::div() >> html::text!("Count: {}", count))
        // >> (count == 0).then_some("Hit `incr` to start counting ...")
        >> {
            if count > 0 {
                (html::div() >> html::text!("Count: {}", count)).boxed()
            } else {
                (html::div() >> "Hit `incr` to start counting ...").boxed()
            }
        }
        >> { html::button().on_click(incr, ()) >> "incr" })
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app().await).await;
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
