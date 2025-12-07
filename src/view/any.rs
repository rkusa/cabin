use crate::View;
use crate::render::Renderer;

pub struct AnyView {
    pub(crate) result: Result<Renderer, crate::Error>,
}

impl View for AnyView {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        match self.result {
            Ok(renderer) => {
                r.append(renderer);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

pub trait IntoAnyView {
    fn into_any_view(self) -> AnyView;
}

impl IntoAnyView for AnyView {
    fn into_any_view(self) -> AnyView {
        self
    }
}
