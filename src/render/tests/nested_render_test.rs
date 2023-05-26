use std::convert::Infallible;
use std::future::ready;

use crate::component::{ComponentId, ServerComponent};
use crate::render::marker::Marker;
use crate::render::PreviousComponent;
use crate::view::fragment;
use crate::{html, Renderer, View, ViewHashTree};

#[tokio::test]
async fn test_unchanged() {
    let component = || {
        ServerComponent::<_, String, _, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(Ok::<_, Infallible>(format!("{state}")))
        })
    };

    let r = (html::div() >> component() >> "text")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,1358177704]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2920442088), // div
            Marker::End(1039239258), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component() >> "text")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2920442088), // div
            Marker::End(1039239258), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component() >> "asdf")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->asdf</div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(1584409650), // text
            Marker::End(2563405466), // div (changed due to text change)
            Marker::End(3441470140), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_changed() {
    let component = |state: u32| {
        ServerComponent::<_, String, _, _, _>::new(
            ComponentId::new("a", "b"),
            state,
            |state: u32| ready(Ok::<_, Infallible>(format!("{state}"))),
        )
    };

    let r = (html::div() >> component(1) >> "text")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[0,3068971186,893607006]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(893607006),  // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(965605196),  // div
            Marker::End(138294499),  // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component(2) >> "text")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,205742900,4053307777]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(4053307777), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2328508059), // div
            Marker::End(2382538632), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_additional() {
    let component = || {
        ServerComponent::<_, String, _, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(Ok::<_, Infallible>(format!("{state}")))
        })
    };

    let r = (html::div() >> "a").render(Renderer::new()).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<div>a</div>");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,           // div
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3545955648), // div
            Marker::End(1340987582), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component() >> "a").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,1358177704]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3360678569), // div (changed due to added component)
            Marker::End(2666260021), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component() >> "a").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3360678569), // div (changed due to added component)
            Marker::End(2666260021), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_replacement() {
    let component = || {
        ServerComponent::<_, String, _, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(Ok::<_, Infallible>(format!("{state}")))
        })
    };

    let r = (html::div() >> "a" >> "b")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, "<div>ab</div>");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,           // div
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(2699768991), // div
            Marker::End(3555049762), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> component() >> "b").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,1358177704]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(879271877),  // div
            Marker::End(309729706),  // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_without_replacement() {
    let component = || {
        ServerComponent::<_, String, _, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(Ok::<_, Infallible>(format!("{state}")))
        })
    };

    let r = (html::div() >> component() >> "a")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,1358177704]}</script>\
        </server-component>a</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3360678569), // div
            Marker::End(2666260021), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> "a").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<div>a</div>");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,           // div
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3545955648), // div (changed due to removed component)
            Marker::End(1340987582), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_by_being_replaced() {
    let component = || {
        ServerComponent::<_, String, _, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(Ok::<_, Infallible>(format!("{state}")))
        })
    };

    let r = (html::div() >> component() >> "b")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3031521205\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,1358177704]}</script>\
        </server-component>b</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3031521205),
            Marker::End(1358177704), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(879271877),  // div
            Marker::End(309729706),  // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div() >> "a" >> "b").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<div>a<!--unchanged--></div>");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,           // div
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(2699768991), // div
            Marker::End(3555049762), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_inner_partial_update() {
    let component = |state: u32| {
        ServerComponent::new(ComponentId::new("a", "b"), state, |state: u32| {
            ready(Ok::<_, Infallible>(
                fragment() >> html::div() >> format!("{state}"),
            ))
        })
    };

    let id = 3031521205;
    let hash_tree: ViewHashTree = vec![
        Marker::Component(id),
        Marker::End(2057549382), // component
        Marker::End(1680570539), // root
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
    let r = component(2).render(r).await.unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<server-component id=\"3031521205\" data-id=\"a::b\"><!--unchanged-->2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,3201766860,0,205742900,1543047475]\
        }</script></server-component>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Component(id),
            Marker::End(1543047475), // component
            Marker::End(3460447620), // root
        ]
        .into()
    );
}
