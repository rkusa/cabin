use std::future::IntoFuture;

use super::RenderFuture;
pub use super::View;
use crate::scope::Scope;
use crate::view::AnyView;

pub trait FutureExt<F, V>
where
    F: IntoFuture<Output = V> + Send,
    F::IntoFuture: Send,
    V: View,
{
    fn into_any_view(self) -> AnyView;
}

impl<F, V> FutureExt<F, V> for F
where
    F: IntoFuture<Output = V> + Send + 'static,
    F::IntoFuture: Send,
    V: View,
{
    fn into_any_view(self) -> AnyView {
        AnyView {
            views: smallvec::smallvec![RenderFuture::Future(Box::pin(async move {
                self.into_future()
                    .await
                    .render(Scope::create_renderer_from_task())
                    .await
            }))],
        }
    }
}
