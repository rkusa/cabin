pub mod any;
pub mod boundary;
pub mod error;
pub(crate) mod internal;
mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt::Write;

pub use any::AnyView;
pub use boundary::Boundary;
use error::ErrorView;
use http_error::HttpError;
pub use iter::IteratorExt;

use crate::render::{Escape, Renderer};
use crate::view::any::IntoAnyView;

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error>;

    fn boundary<Args>(self, args: Args) -> Boundary<Args>
    where
        Self: Sized + IntoAnyView,
    {
        Boundary::new(self.into_any_view(), args)
    }
}

impl View for () {
    fn render(self, _r: &mut Renderer) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<'s> View for &'s str {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        Cow::Borrowed(self).render(r)
    }
}

impl<'s> View for Cow<'s, str> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let mut txt = r.text();
        Escape::content(&mut txt)
            .write_str(&self)
            .map_err(crate::error::InternalError::from)?;
        txt.end();
        Ok(())
    }
}

impl View for String {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        Cow::<'static, str>::Owned(self).render(r)
    }
}

impl<'s> View for &'s String {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        Cow::Borrowed(self.as_str()).render(r)
    }
}

impl<'v, V> View for Option<V>
where
    V: View,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        match self {
            Some(i) => i.render(r),
            None => Ok(()),
        }
    }
}

impl<V, E> View for Result<V, E>
where
    V: View,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: ErrorView,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        match self {
            Ok(v) => v.render(r),
            Err(err) => {
                if err.should_render() {
                    let c = r.empty_context();
                    err.into_view(&c).render(r)
                } else {
                    Err(crate::Error::from(
                        Box::<dyn HttpError + Send + 'static>::from(err),
                    ))
                }
            }
        }
    }
}

impl<V> View for Vec<V>
where
    V: View,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        for i in self {
            i.render(r)?;
        }
        Ok(())
    }
}
