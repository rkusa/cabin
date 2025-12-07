use crate::View;
use crate::render::Renderer;
use crate::view::AnyView;
use crate::view::any::IntoAnyView;
use crate::view::internal::Internal;

pub struct Fragment(Internal<FragmentBuilder>);

struct FragmentBuilder {
    renderer: Renderer,
}

impl Fragment {
    pub(crate) fn new(renderer: Renderer) -> Self {
        Self(Internal::new(FragmentBuilder { renderer }))
    }

    pub fn child(mut self, child: impl View) -> Self {
        self.append_child(child);
        self
    }

    pub(crate) fn append_child(&mut self, child: impl View) {
        let Some(builder) = self.0.builder_mut() else {
            return;
        };

        if let Err(err) = child.render(&mut builder.renderer) {
            self.0.errored(err);
        }
    }

    pub(crate) fn render(self) -> Result<Renderer, crate::Error> {
        self.0.take_builder().map(|b| b.renderer)
    }

    pub fn any(self) -> AnyView {
        AnyView {
            result: self.render(),
        }
    }
}

impl View for Fragment {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl IntoAnyView for Fragment {
    fn into_any_view(self) -> AnyView {
        self.any()
    }
}
