use std::future::ready;
use std::str::FromStr;

use crate::component::id::NanoId;
use crate::component::{ComponentId, ServerComponent};
use crate::render::marker::Marker;
use crate::render::PreviousComponent;
use crate::{html, view, Renderer, View, ViewHashTree};

#[tokio::test]
async fn test_unchanged() {
    let component = || {
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div(view![component(), "text"])
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[-1,1212501170,1918658755]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(4117075044), // div
            Marker::End(2644152265), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div(view![component(), "text"])
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(4117075044), // div
            Marker::End(2644152265), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div(view![component(), "asdf"])
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->asdf</div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(1584409650), // text
            Marker::End(2668202860), // div (changed due to text change)
            Marker::End(1959759848), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_changed() {
    let component = |state: u32| {
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), state, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div(view![component(1), "text"])
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[-1,3068971186,2057549382]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(2057549382), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2957461891), // div
            Marker::End(2418331084), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div(view![component(2), "text"])
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[-1,205742900,1581202394]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1581202394), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2930295086), // div
            Marker::End(295470050),  // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_additional() {
    let component = || {
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div("a").render(Renderer::new()).await.unwrap();
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
    let r = html::div(view![component(), "a"]).render(r).await.unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[-1,1212501170,1918658755]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3413073872), // div (changed due to added component)
            Marker::End(3770318169), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div(view![component(), "a"]).render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3413073872), // div (changed due to added component)
            Marker::End(3770318169), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_replacement() {
    let component = || {
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div(view!["a", "b"])
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
    let r = html::div(view![component(), "b"]).render(r).await.unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[-1,1212501170,1918658755]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(1982657642), // div
            Marker::End(2906381823), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_without_replacement() {
    let component = || {
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div(view![component(), "a"])
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[-1,1212501170,1918658755]}</script>\
        </server-component>a</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(3413073872), // div
            Marker::End(3770318169), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div("a").render(r).await.unwrap();
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
        ServerComponent::<_, String, _, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div(view![component(), "b"])
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[-1,1212501170,1918658755]}</script>\
        </server-component>b</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap()),
            Marker::End(1918658755), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(1982657642), // div
            Marker::End(2906381823), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div(view!["a", "b"]).render(r).await.unwrap();
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
            ready(view![html::div(()), format!("{}", state)])
        })
    };

    let id = NanoId::from_str("ZJGbMZEVVDBciW-4k8Ld0").unwrap();
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
        "<server-component id=\"ZJGbMZEVVDBciW-4k8Ld0\" data-id=\"a::b\"><!--unchanged-->2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[-1,3201766860,-1,205742900,2989134655]\
        }</script></server-component>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Component(id),
            Marker::End(2989134655), // component
            Marker::End(3009041063), // root
        ]
        .into()
    );
}
