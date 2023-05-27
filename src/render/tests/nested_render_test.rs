use std::convert::Infallible;

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

#[tokio::test]
async fn test_unchanged() {
    let r = html::div::<_, ()>((TestComponent::restore(()), "text"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,813171319]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(3670748561), // div
            Marker::End(3476836677), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((TestComponent::restore(()), "text"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(3670748561), // div
            Marker::End(3476836677), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((TestComponent::restore(()), "asdf"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->asdf</div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(1584409650), // text
            Marker::End(3213113940), // div (changed due to text change)
            Marker::End(2607991424), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_changed() {
    let r = html::div::<_, ()>((TestComponent::restore_or((), TestComponent(1)), "text"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">1<script \
        type=\"application/json\">{\"state\":1,\"hashTree\":[0,3068971186,1613667077]}</script>\
        </server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(1613667077), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(3100873811), // div
            Marker::End(2799658939), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((TestComponent::restore_or((), TestComponent(2)), "text"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,205742900,4079019887]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(4079019887), // component
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(1938471606), // div
            Marker::End(1557931932), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_additional() {
    let r = html::div::<_, ()>("a")
        .render(Renderer::new())
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
    let r = html::div::<_, ()>((TestComponent::restore(()), "a"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,813171319]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(1134459290), // div (changed due to added component)
            Marker::End(939669540),  // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((TestComponent::restore(()), "a"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(1134459290), // div (changed due to added component)
            Marker::End(939669540),  // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_replacement() {
    let r = html::div::<_, ()>(("a", "b"))
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
    let r = html::div::<_, ()>((TestComponent::restore(()), "b"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,813171319]}</script>\
        </server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(3676495876), // div
            Marker::End(2936353090), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_without_replacement() {
    let r = html::div::<_, ()>((TestComponent::restore(()), "a"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,813171319]}</script>\
        </server-component>a</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(1134459290), // div
            Marker::End(939669540),  // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>("a").render(r).await.unwrap();
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
    let r = html::div::<_, ()>((TestComponent::restore(()), "b"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component id=\"3704386324\" data-id=\"a::b\">0<script \
        type=\"application/json\">{\"state\":0,\"hashTree\":[0,1212501170,813171319]}</script>\
        </server-component>b</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component(3704386324),
            Marker::End(813171319),  // component
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(3676495876), // div
            Marker::End(2936353090), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>(("a", "b")).render(r).await.unwrap();
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
    let r = View::<()>::render(
        Restored::new(id, TestComponent(2)).with_previous_hash_tree(with_previous_hash_tree.into()),
        r,
    )
    .await
    .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<server-component id=\"3704386324\" data-id=\"a::b\"><!--unchanged-->2<script \
        type=\"application/json\">{\"state\":2,\"hashTree\":[0,3201766860,0,205742900,1848517953]\
        }</script></server-component>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Component(id),
            Marker::End(1848517953), // component
            Marker::End(460124765),  // root
        ]
        .into()
    );
}
