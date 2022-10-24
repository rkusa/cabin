use std::future::ready;

use crate::component::{ComponentId, ServerComponent};
use crate::render::marker::Marker;
use crate::{html, Renderer, View};

#[tokio::test]
async fn test_unchanged() {
    let component = || {
        ServerComponent::<_, String, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div::<_, ()>((component(), "text"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component data-id=\"a::b\"><script type=\"application/json\">{\"state\":0,\
        \"hashTree\":[\"s\",1212501170,1918658755]}</script>0</server-component>text</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2250130625), // div
            Marker::End(3433365191), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((component(), "text"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(2564554603), // text
            Marker::End(2250130625), // div
            Marker::End(3433365191), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((component(), "asdf"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->asdf</div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(1584409650), // text
            Marker::End(1087932597), // div (changed due to text change)
            Marker::End(3845688878), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added() {
    let component = || {
        ServerComponent::<_, String, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

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
    let r = html::div::<_, ()>((component(), "a"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component data-id=\"a::b\"><script type=\"application/json\">{\"state\":0,\
        \"hashTree\":[\"s\",1212501170,1918658755]}</script>0</server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(2793600937), // div (changed due to added component)
            Marker::End(4012198563), // root
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>((component(), "a"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, "<!--unchanged-->");
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(2793600937), // div
            Marker::End(4012198563), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_added_as_replacement() {
    let component = || {
        ServerComponent::<_, String, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

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
    let r = html::div::<_, ()>((component(), "b"))
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component data-id=\"a::b\"><script type=\"application/json\">{\"state\":0,\
        \"hashTree\":[\"s\",1212501170,1918658755]}</script>0</server-component><!--unchanged--></div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(192546232),  // div
            Marker::End(1559871055), // root
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed() {
    let component = || {
        ServerComponent::<_, String, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div::<_, ()>((component(), "a"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component data-id=\"a::b\"><script type=\"application/json\">{\"state\":0,\
        \"hashTree\":[\"s\",1212501170,1918658755]}</script>0</server-component>a</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(1426945110), // text
            Marker::End(2793600937), // div
            Marker::End(4012198563), // root
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
    let component = || {
        ServerComponent::<_, String, _>::new(ComponentId::new("a", "b"), 0, |state: u32| {
            ready(format!("{}", state))
        })
    };

    let r = html::div::<_, ()>((component(), "b"))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        "<div><server-component data-id=\"a::b\"><script type=\"application/json\">{\"state\":0,\
            \"hashTree\":[\"s\",1212501170,1918658755]}</script>0</server-component>b</div>"
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // div
            Marker::Component,
            Marker::Start,           // text
            Marker::End(2718739903), // text
            Marker::End(192546232),  // div
            Marker::End(1559871055), // root
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
