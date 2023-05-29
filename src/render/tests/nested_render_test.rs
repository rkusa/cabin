use std::convert::Infallible;
use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::component::{Component, ComponentId, PublicComponent};
use crate::render::marker::Marker;
use crate::{html, Renderer, Restored, View, ViewHashTree};

#[derive(Debug, Default, Hash, Serialize, Deserialize)]
struct TestComponent(u32);

impl PublicComponent for TestComponent {
    fn id() -> ComponentId {
        ComponentId::new("a", "b")
    }
}

impl Component for TestComponent {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _: Self::Event) {}

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok(html::text!("{}", self.0))
    }
}

async fn run_in_local_set<T>(f: impl Future<Output = T>) -> T {
    let local = tokio::task::LocalSet::new();
    local.run_until(f).await
}

#[tokio::test]
async fn test_unchanged() {
    let r = run_in_local_set(
        html::div::<_, ()>((TestComponent::restore(()), "text")).render(Renderer::new()),
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,3733041516]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(995587091),  // div
            Marker::End(3874394579), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(html::div::<_, ()>((TestComponent::restore(()), "text")).render(r))
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(995587091),  // div
            Marker::End(3874394579), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(html::div::<_, ()>((TestComponent::restore(()), "asdf")).render(r))
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->asdf</div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(1584409650), // text
            Marker::End(1762692303), // div (changed due to text change)
            Marker::End(1212963887), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_changed() {
    let r = run_in_local_set(
        html::div::<_, ()>((TestComponent::restore_or((), TestComponent(1)), "text"))
            .render(Renderer::new()),
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[0,3068971186,1843171630]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(1843171630), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(258514324),  // div
            Marker::End(1929089212), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(
        html::div::<_, ()>((TestComponent::restore_or((), TestComponent(2)), "text")).render(r),
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,205742900,1362020072]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(1362020072), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2940963625), // div
            Marker::End(2050439521), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_additional() {
    let r = run_in_local_set(html::div::<_, ()>("a").render(Renderer::new()))
        .await
        .unwrap();
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
    let r = run_in_local_set(html::div::<_, ()>((TestComponent::restore(()), "a")).render(r))
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,3733041516]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(782395811),  // div (changed due to added component)
            Marker::End(1189561557), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(html::div::<_, ()>((TestComponent::restore(()), "a")).render(r))
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(782395811),  // div (changed due to added component)
            Marker::End(1189561557), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_replacement() {
    let r = run_in_local_set(html::div::<_, ()>(("a", "b")).render(Renderer::new()))
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
    let r = run_in_local_set(html::div::<_, ()>((TestComponent::restore(()), "b")).render(r))
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,3733041516]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(311788293),  // div
            Marker::End(1558363454), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_without_replacement() {
    let r = run_in_local_set(
        html::div::<_, ()>((TestComponent::restore(()), "a")).render(Renderer::new()),
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,3733041516]}</script>\
        </server-component>a</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(782395811),  // div
            Marker::End(1189561557), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(html::div::<_, ()>("a").render(r))
        .await
        .unwrap();
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
    let r = run_in_local_set(
        html::div::<_, ()>((TestComponent::restore(()), "b")).render(Renderer::new()),
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,3733041516]}</script>\
        </server-component>b</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(3733041516), // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(311788293),  // div
            Marker::End(1558363454), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = run_in_local_set(html::div::<_, ()>(("a", "b")).render(r))
        .await
        .unwrap();
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
    #[derive(Debug, Default, Hash, Serialize, Deserialize)]
    struct TestComponent(u32);

    impl PublicComponent for TestComponent {
        fn id() -> ComponentId {
            ComponentId::new("a", "b")
        }
    }

    impl Component for TestComponent {
        type Event = ();
        type Error = Infallible;

        async fn update(&mut self, _: Self::Event) {}

        async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
            Ok((html::div(()), html::text!("{}", self.0)))
        }
    }

    let id = 3704386324;
    let hash_tree: ViewHashTree = vec![
        Marker::Component(id),
        Marker::End(2057549382), // component
        Marker::End(1680570539), // root
    ]
    .into();
    let with_previous_hash_tree = vec![
        Marker::Start,
        Marker::End(3201766860), // div
        Marker::Start,
        Marker::End(3068971186), // text
        Marker::End(878693578),  // root
    ];
    let r = Renderer::from_previous_tree(hash_tree.clone());
    let r = run_in_local_set(View::<()>::render(
        Restored::new(id, TestComponent(2)).with_previous_hash_tree(with_previous_hash_tree.into()),
        r,
    ))
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<server-component id=\"3704386324\" data-id=\"a::b\"><!--unchanged-->2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,3201766860,0,205742900,2874101245]\
        }</script></server-component>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Component(id),
            Marker::End(2874101245), // component
            Marker::End(2866621995), // root
        ]
        .into()
    );
}
