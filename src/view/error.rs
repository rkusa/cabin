use http_error::AnyHttpError;

use crate::{Context, View};

pub trait ErrorView {
    fn into_view(self, c: &Context) -> impl View;
    fn should_render(&self) -> bool {
        true
    }
}

impl ErrorView for AnyHttpError {
    fn into_view(self, _: &Context) -> impl View {
        ()
    }

    fn should_render(&self) -> bool {
        false
    }
}
