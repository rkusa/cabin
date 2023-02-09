use std::convert::Infallible;
use std::future::ready;
use std::str::FromStr;

use crate::component::id::NanoId;
use crate::component::{ComponentId, ServerComponent};
use crate::previous::{previous, FromPrevious};
use crate::render::marker::Marker;
use crate::render::PreviousComponent;
use crate::{html, view, Renderer, View, ViewHashTree};

#[tokio::test]
async fn test_previous_state() {
    fn component(state: impl FromPrevious<u32> + 'static) -> impl View {
        ServerComponent::new(ComponentId::new("a", "b"), state, |state: u32| {
            ready(Ok::<_, Infallible>(view![
                html::div(()),
                format!("{state}")
            ]))
        })
    }

    let id = NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap();
    let hash_tree: ViewHashTree = vec![
        Marker::Component(id),
        Marker::End(1848809075), // component
        Marker::End(1079271567), // root
    ]
    .into();
    let r = Renderer::from_previous_tree(hash_tree.clone()).with_descendants(
        [(
            id,
            PreviousComponent {
                state: serde_json::value::to_raw_value(&1).unwrap(),
                hash_tree: vec![
                    Marker::Start,
                    Marker::End(3201766860), // div
                    Marker::Start,
                    Marker::End(3068971186), // text
                    Marker::End(878693578),  // root
                ]
                .into(),
            },
        )]
        .into_iter()
        .collect(),
    );
    let r = component(previous(|n: u32| n)).render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(out.hash_tree, hash_tree);
}

#[tokio::test]
async fn test_previous_default() {
    fn component(state: impl FromPrevious<u32> + 'static) -> impl View {
        ServerComponent::new(ComponentId::new("a", "b"), state, |state: u32| {
            ready(Ok::<_, Infallible>(view![
                html::div(()),
                format!("{state}")
            ]))
        })
    }

    let r = component(previous(|n: u32| n + 1))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\"><div></div>1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[-1,3201766860,-1,3068971186,\
        1848809075]}</script></server-component>"
    );
}
