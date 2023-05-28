#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    arbitrary_self_types
)]
#![allow(incomplete_features)]

use std::convert::Infallible;
use std::hash::Hash;
use std::marker::PhantomData;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use cabin::component::{Component, PublicComponent};
use cabin::{cabin_scripts, cabin_stylesheets, html, Restored, View};

async fn app() -> impl View {
    (cabin_stylesheets(), cabin_scripts(), root().await)
}

async fn root() -> Result<impl View, Infallible> {
    // TODO: content is gone when directly updating component
    Ok(Dialog::restore((), "Hello World").opened(true))
}

#[derive(Default, Hash, serde::Serialize, serde::Deserialize, PublicComponent)]
struct Dialog<V = (), Ev = ()> {
    opened: bool,
    #[serde(skip)]
    content: V,
    #[serde(skip)]
    marker: PhantomData<Ev>,
}

impl<V> Dialog<V> {
    // TODO: well, how bad is `self: Restored<Self>`?
    pub fn opened(mut self: Restored<Self>, opened: bool) -> Restored<Self> {
        self.opened = opened;
        self
    }
}

impl Dialog<()> {
    pub fn restore<V, Ev>(id: impl Hash, content: V) -> Restored<Dialog<V, Ev>> {
        let dialog: Restored<Dialog<()>> = Component::restore(id);
        dialog.map(|dialog| Dialog {
            opened: dialog.opened,
            content,
            marker: PhantomData,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
enum DialogEvent {
    Open,
    Close,
}

impl<V: View<Ev>, Ev> Component for Dialog<V, Ev> {
    type Event = DialogEvent;
    type Error = Infallible;

    async fn update(&mut self, event: Self::Event) {
        match event {
            DialogEvent::Open => self.opened = true,
            DialogEvent::Close => self.opened = false,
        }
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok((
            html::dialog((
                self.content.coerce(),
                html::button("close").on_click(DialogEvent::Close),
            ))
            .open(self.opened),
            html::button("open").on_click(DialogEvent::Open),
        ))
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
