use http_error::AnyHttpError;

use crate::View;

pub trait ErrorView {
    fn into_view(self) -> impl View;
    fn should_render(&self) -> bool {
        true
    }
}

impl ErrorView for AnyHttpError {
    fn into_view(self) -> impl View {}

    fn should_render(&self) -> bool {
        false
    }
}
