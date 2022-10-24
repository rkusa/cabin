use std::hash::Hasher;

use twox_hash::XxHash32;

use super::marker::Marker;
use super::Renderer;
use crate::{html, View};

#[tokio::test]
async fn test_server_render_basic() {
    let r = html::div::<_, ()>("test")
        .attr("class", "bg-black")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div class="bg-black">test</div>"#);

    let mut root_hasher = XxHash32::default();
    let mut div_hasher = XxHash32::default();
    div_hasher.write(b"div");
    div_hasher.write(b"class");
    div_hasher.write(b"bg-black");

    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // <div>
            Marker::Start, // text
            Marker::End({
                let mut hasher = XxHash32::default();
                hasher.write(b"test");
                let hash = hasher.finish() as u32;
                div_hasher.write_u32(hash);
                hash
            }),
            Marker::End({
                let hash = div_hasher.finish() as u32;
                root_hasher.write_u32(hash);
                hash
            }), // </div>
            Marker::End(root_hasher.finish() as u32)
        ]
        .into()
    );
}

#[tokio::test]
async fn test_server_render_empty() {
    let r = html::div::<_, ()>(())
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div></div>"#);
}

#[tokio::test]
async fn test_server_render_nested() {
    let r = html::div::<_, ()>((
        html::div("red").attr("class", "bg-red"),
        html::div("green").attr("class", "bg-green"),
    ))
    .render(Renderer::new())
    .await
    .unwrap();
    let out = r.end();

    assert_eq!(
        out.view,
        r#"<div><div class="bg-red">red</div><div class="bg-green">green</div></div>"#
    );

    let mut root_hasher = XxHash32::default();
    let mut wrapper_hasher = XxHash32::default();
    wrapper_hasher.write(b"div");
    let mut div1_hasher = XxHash32::default();
    div1_hasher.write(b"div");
    div1_hasher.write(b"class");
    div1_hasher.write(b"bg-red");
    let mut div2_hasher = XxHash32::default();
    div2_hasher.write(b"div");
    div2_hasher.write(b"class");
    div2_hasher.write(b"bg-green");

    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start, // <div>
            Marker::Start, // <div class="bg-red">
            Marker::Start, // text "red"
            Marker::End({
                let mut hasher = XxHash32::default();
                hasher.write(b"red");
                let hash = hasher.finish() as u32;
                div1_hasher.write_u32(hash);
                hash
            }),
            Marker::End({
                let hash = div1_hasher.finish() as u32;
                wrapper_hasher.write_u32(hash);
                hash
            }), // </div>
            Marker::Start, // <div class="bg-green">
            Marker::Start, // text "green"
            Marker::End({
                let mut hasher = XxHash32::default();
                hasher.write(b"green");
                let hash = hasher.finish() as u32;
                div2_hasher.write_u32(hash);
                hash
            }),
            Marker::End({
                let hash = div2_hasher.finish() as u32;
                wrapper_hasher.write_u32(hash);
                hash
            }), // </div>
            Marker::End({
                let hash = wrapper_hasher.finish() as u32;
                root_hasher.write_u32(hash);
                hash
            }), // </div>
            Marker::End(root_hasher.finish() as u32)
        ]
        .into()
    );
}

#[tokio::test]
async fn test_unchanged() {
    let r = html::div::<_, ()>(("test", html::div(())))
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div>test<div></div></div>"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(1042293711),
            Marker::Start,
            Marker::End(3201766860),
            Marker::End(786131022),
            Marker::End(331105061),
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>(("test", html::div(())))
        .attr("class", "bg-black")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(
        out.view,
        r#"<div class="bg-black"><!--unchanged--><!--unchanged--></div>"#
    );
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(1042293711),
            Marker::Start,
            Marker::End(3201766860),
            Marker::End(3488591968),
            Marker::End(2438041366),
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = html::div::<_, ()>(("test", html::div(())))
        .attr("class", "bg-black")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(1042293711),
            Marker::Start,
            Marker::End(3201766860),
            Marker::End(3488591968),
            Marker::End(2438041366),
        ]
        .into()
    );
}

#[tokio::test]
async fn test_new_items() {
    let r = (html::div::<_, ()>("1"), "E")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div>1</div>E"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::End(3652438263),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(1782583075),
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div::<_, ()>(("1", "2")), "E")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->2</div><!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::Start,
            Marker::End(205742900),
            Marker::End(692489137),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(4063303603),
        ]
        .into()
    );
}

#[tokio::test]
async fn test_removed_items() {
    let r = (html::div::<_, ()>(("1", "2")), "E")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div>12</div>E"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::Start,
            Marker::End(205742900),
            Marker::End(692489137),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(4063303603),
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div::<_, ()>("1"), "E").render(r).await.unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged--></div><!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::End(3652438263),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(1782583075),
        ]
        .into()
    );
}

#[tokio::test]
async fn test_new_item_same_value() {
    let r = (html::div::<_, ()>("1"), "E")
        .render(Renderer::new())
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div>1</div>E"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::End(3652438263),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(1782583075),
        ]
        .into()
    );

    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = (html::div::<_, ()>(("1", "1")), "E")
        .render(r)
        .await
        .unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div><!--unchanged-->1</div><!--unchanged-->"#);
    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Start,
            Marker::Start,
            Marker::End(3068971186),
            Marker::Start,
            Marker::End(3068971186),
            Marker::End(1614397666),
            Marker::Start,
            Marker::End(727368102),
            Marker::End(4204980819),
        ]
        .into()
    );
}
