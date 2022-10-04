#![allow(clippy::let_and_return)]

use std::fmt;
use std::future::Future;
use std::hash::Hasher;
use std::pin::Pin;

use twox_hash::XxHash32;

use super::marker::Marker;
use super::Renderer;
use crate::View;

struct Child<R, F>
where
    R: FnOnce(Renderer) -> F + Send,
    F: Future<Output = Renderer> + Send,
{
    render: R,
}

impl<R, F> View<()> for Child<R, F>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    R: FnOnce(Renderer) -> F + Send + 'static,
    F: Future<Output = Renderer> + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move { Ok((self.render)(r).await) })
    }
}

fn child<R, F>(f: R) -> Child<R, F>
where
    R: FnOnce(Renderer) -> F + Send,
    F: Future<Output = Renderer> + Send,
{
    Child { render: f }
}

#[tokio::test]
async fn test_server_render_basic() {
    let r = Renderer::new();
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    let r = el.content::<()>("test").await.unwrap();
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
    let r = Renderer::new();
    let r = r.element("div").unwrap().end().unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div></div>"#);
}

#[tokio::test]
async fn test_server_render_nested() {
    let r = Renderer::new();
    let wrapper = r.element("div").unwrap();
    let r = wrapper
        .content(child(|r| async move {
            let r = {
                let mut el = r.element("div").unwrap();
                el.attribute("class", "bg-red").unwrap();
                el.content::<()>("red").await.unwrap()
            };
            let r = {
                let mut el = r.element("div").unwrap();
                el.attribute("class", "bg-green").unwrap();
                el.content::<()>("green").await.unwrap()
            };
            r
        }))
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
    let r = Renderer::new();
    let el = r.element("div").unwrap();
    let r = el
        .content(child(|mut r| async move {
            r.text("test").unwrap();
            r.element("div").unwrap().end().unwrap()
        }))
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
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    let r = el
        .content(child(|mut r| async move {
            r.text("test").unwrap();
            r.element("div").unwrap().end().unwrap()
        }))
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
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    let r = el
        .content(child(|mut r| async move {
            r.text("test").unwrap();
            r.element("div").unwrap().end().unwrap()
        }))
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
    let r = Renderer::new();
    let el = r.element("div").unwrap();
    let mut r = el.content::<()>("1").await.unwrap();
    r.text("E").unwrap();
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
    let el = r.element("div").unwrap();
    let mut r = el
        .content(child(|mut r| async move {
            r.text("1").unwrap();
            r.text("2").unwrap();
            r
        }))
        .await
        .unwrap();
    r.text("E").unwrap();
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
    let r = Renderer::new();
    let el = r.element("div").unwrap();
    let mut r = el
        .content(child(|mut r| async move {
            r.text("1").unwrap();
            r.text("2").unwrap();
            r
        }))
        .await
        .unwrap();
    r.text("E").unwrap();
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
    let el = r.element("div").unwrap();
    let mut r = el.content::<()>("1").await.unwrap();
    r.text("E").unwrap();
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
    let r = Renderer::new();
    let el = r.element("div").unwrap();
    let mut r = el.content::<()>("1").await.unwrap();
    r.text("E").unwrap();
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
    let el = r.element("div").unwrap();
    let mut r = el
        .content(child(|mut r| async move {
            r.text("1").unwrap();
            r.text("1").unwrap();
            r
        }))
        .await
        .unwrap();
    r.text("E").unwrap();
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
