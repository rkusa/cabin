use std::net::SocketAddr;
use std::sync::LazyLock;

use cabin::cabin_scripts;
use cabin::element::ElementProxy;
use cabin::h::fragment;
use cabin::prelude::*;
use cabin_tailwind::prelude::*;
use cabin_tailwind::registry::{StyleRegistry, StyleSheet};
use http::Request;
use tokio::net::TcpListener;

mod badge {
    use cabin::attribute::WithAttribute;
    use cabin::element::ElementProxy;
    use cabin::html::Element;
    use cabin::html::elements::span::marker;
    use cabin_tailwind::Class;

    use super::*;

    pub fn badge() -> Badge {
        Badge {
            element: Element::new("span"),
            variant: Variant::default(),
            is_solid: false,
        }
    }

    pub struct Badge {
        element: Element<marker::Span>,
        variant: Variant,
        is_solid: bool,
    }

    impl ElementProxy<marker::Span> for Badge {
        fn into_element(self) -> Element<marker::Span> {
            self.element
                .with_attribute(base_css().append(self.variant.into_classname(self.is_solid)))
        }

        fn child<'c>(
            self,
            child: impl cabin::element::IntoChild<'c, marker::Span> + 'c,
        ) -> cabin::element::ElementContent<marker::Span, ()> {
            self.into_element().child(child)
        }
    }

    impl WithAttribute for Badge {
        fn with_attribute(mut self, attr: impl cabin::attribute::Attribute) -> Self {
            self.element = self.element.with_attribute(attr);
            self
        }
    }

    impl View for Badge {
        fn render(self, r: &mut cabin::render::Renderer) -> Result<(), cabin::Error> {
            self.into_element().render(r)
        }
    }

    impl Badge {
        pub fn solid(mut self) -> Self {
            self.is_solid = true;
            self
        }

        pub fn success(mut self) -> Self {
            self.variant = Variant::Success;
            self
        }

        pub fn danger(mut self) -> Self {
            self.variant = Variant::Danger;
            self
        }
    }

    #[derive(Default, Clone, Copy)]
    #[allow(unused)]
    pub enum Variant {
        #[default]
        Success,
        Danger,
    }

    fn base_css() -> Class {
        tw0![
            tw::INLINE_FLEX,
            tw::items::CENTER,
            tw::px::pxf(2.5),
            tw::py::pxf(0.5),
            tw::rounded::SM,
            tw::text::XS,
            tw::h::px(18),
            tw::font::MEDIUM,
            tw::UPPERCASE,
            tw::whitespace::NOWRAP
        ]
    }

    impl Variant {
        fn into_classname(self, is_solid: bool) -> Class {
            if is_solid {
                match self {
                    Variant::Success => tw0![tw::bg::GREEN_600, tw::text::WHITE],
                    Variant::Danger => tw0![tw::bg::RED_600, tw::text::WHITE],
                }
            } else {
                match self {
                    Variant::Success => tw0![tw::bg::GREEN_200, tw::text::GREEN_800],
                    Variant::Danger => tw0![tw::bg::RED_200, tw::text::RED_800],
                }
            }
        }
    }
}

async fn app() -> impl View {
    document(
        fragment()
            .child(badge::badge().solid().success().child("SUCCESS"))
            .child(badge::badge().success().child("SUCCESS"))
            .child(badge::badge().solid().danger().child("DANGER"))
            .child(badge::badge().danger().child("DANGER")),
    )
}

fn document(content: impl View) -> impl View {
    h::fragment().child(h::doctype()).child(
        h::html()
            .child(h::head().child(STYLE_SHEET.link()).child(cabin_scripts()))
            .child(h::body().child(content)),
    )
}

cabin::BOUNDARIES!();

cabin_tailwind::STYLES!();
static STYLE_SHEET: LazyLock<StyleSheet> =
    LazyLock::new(|| StyleRegistry::default().with(&STYLES).build(true));

#[tokio::main]
async fn main() {
    let filter =
        tracing_subscriber::filter::filter_fn(|metadata| metadata.target().starts_with("cabin"));
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::new().pretty())
        .with(filter)
        .init();

    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer(&BOUNDARIES))
        .layer(cabin_service::livereload::layer())
        .layer(cabin_service::assets::layer_with_stylesheet(&STYLE_SHEET));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
