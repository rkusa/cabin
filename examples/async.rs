#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use cabin::component::{Component, PublicComponent};
use cabin::html::events::InputValue;
use cabin::view::IteratorExt;
use cabin::{cabin_scripts, cabin_stylesheets, html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (
        cabin_stylesheets(),
        cabin_scripts(),
        Search::restore_or_else((), || Search::new("Ge")),
    )
}

#[derive(Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Search {
    query: Cow<'static, str>,
}

impl Search {
    fn new(query: &'static str) -> Self {
        Self {
            query: query.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum SearchEvent {
    Search(InputValue),
}

impl Component for Search {
    type Event = SearchEvent;
    type Error = Infallible;

    async fn update(&mut self, event: Self::Event) {
        match event {
            SearchEvent::Search(query) => self.query = query.into(),
        }
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        let items = search_countries(&self.query).await;

        Ok(html::div((
            html::div(
                html::input()
                    .attr("value", self.query)
                    .on_input(|ev| SearchEvent::Search(ev.value)),
            ),
            html::div(html::ul(items.into_iter().map(html::li).into_view())),
        )))
    }
}

async fn search_countries(query: &str) -> Vec<Cow<'static, str>> {
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Source: https://github.com/umpirsky/country-list/blob/master/data/en_US/country.txt
    const COUNTRIES: &str = include_str!("./countries.txt");
    let query = query.to_lowercase();
    COUNTRIES
        .lines()
        .filter(|country| country.to_lowercase().contains(&query))
        .map(Cow::Borrowed)
        .collect()
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
