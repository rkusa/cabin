use std::future::{Future, IntoFuture};
use std::marker::PhantomData;

use tokio::task::JoinHandle;

pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<F, V, Ev>
where
    F: IntoFuture<Output = V>,
    V: View<Ev>,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V, Ev>;
}

impl<F, V, Ev> FutureExt<F, V, Ev> for F
where
    F: IntoFuture<Output = V>,
    V: View<Ev>,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V, Ev> {
        FutureView {
            state: State::Stored(self.into_future()),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V, Ev>
where
    F: Future,
{
    state: State<F>,
    marker: PhantomData<(V, Ev)>,
}

enum State<F>
where
    F: Future,
{
    Stored(F),
    Primed(JoinHandle<F::Output>),
    Intermediate,
}

impl<F, V, Ev> View<Ev> for FutureView<F, V, Ev>
where
    F: Future<Output = V> + 'static,
    V: View<Ev> + 'static,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let view = match self.state {
            State::Stored(f) => f.await,
            State::Primed(f) => f.await.unwrap(), // TODO: handle JoinError?
            State::Intermediate => unreachable!(),
        };
        view.render(r).await
    }

    fn prime(&mut self) {
        let s = std::mem::replace(&mut self.state, State::Intermediate);
        let _ = std::mem::replace(
            &mut self.state,
            match s {
                State::Stored(f) => State::Primed(tokio::task::spawn_local(f)),
                State::Primed(f) => State::Primed(f),
                State::Intermediate => unreachable!(),
            },
        );
    }
}
