use std::convert::Infallible;
use std::future::ready;

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

    let id = 97889413;
    let hash_tree: ViewHashTree = vec![
        Marker::Component(id),
        Marker::End(280478390),  // component
        Marker::End(3020632720), // root
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
                    Marker::End(280478390),  // root
                ]
                .into(),
            },
        )]
        .into_iter()
        .collect(),
    );
    let r = component(previous((), |n: u32| n)).render(r).await.unwrap();
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

    let r = component(previous((), |n: u32| n + 1))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<server-component id=\"97889413\" data-id=\"a::b\"><div></div>1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[0,3201766860,0,3068971186,\
        280478390]}</script></server-component>"
    );
}
