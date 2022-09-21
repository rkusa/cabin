use std::hash::Hasher;

use super::marker::Marker;
use super::Renderer;
use twox_hash::XxHash32;

#[test]
fn test_server_render_basic() {
    let mut r = Renderer::new();
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    el.content().unwrap().text("test").unwrap();
    el.end().unwrap();
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

#[test]
fn test_server_render_empty() {
    let mut r = Renderer::new();
    r.element("div").unwrap().end().unwrap();
    let out = r.end();
    assert_eq!(out.view, r#"<div></div>"#);
}

#[test]
fn test_server_render_nested() {
    let mut r = Renderer::new();
    let mut wrapper = r.element("div").unwrap();
    let content = wrapper.content().unwrap();
    {
        let mut el = content.element("div").unwrap();
        el.attribute("class", "bg-red").unwrap();
        el.content().unwrap().text("red").unwrap();
        el.end().unwrap();
    }
    {
        let mut el = content.element("div").unwrap();
        el.attribute("class", "bg-green").unwrap();
        el.content().unwrap().text("green").unwrap();
        el.end().unwrap();
    }
    wrapper.end().unwrap();
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

#[test]
fn test_unchanged() {
    let mut r = Renderer::new();
    let mut el = r.element("div").unwrap();
    el.content().unwrap().text("test").unwrap();
    el.content().unwrap().element("div").unwrap().end().unwrap();
    el.end().unwrap();
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

    let mut r = Renderer::from_previous_tree(out.hash_tree);
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    el.content().unwrap().text("test").unwrap();
    el.content().unwrap().element("div").unwrap().end().unwrap();
    el.end().unwrap();
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

    let mut r = Renderer::from_previous_tree(out.hash_tree);
    let mut el = r.element("div").unwrap();
    el.attribute("class", "bg-black").unwrap();
    el.content().unwrap().text("test").unwrap();
    el.content().unwrap().element("div").unwrap().end().unwrap();
    el.end().unwrap();
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

#[test]
fn test_new_items() {
    let mut r = Renderer::new();
    let mut el = r.element("div").unwrap();
    el.content().unwrap().text("1").unwrap();
    el.end().unwrap();
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

    let mut r = Renderer::from_previous_tree(out.hash_tree);
    let mut el = r.element("div").unwrap();
    el.content().unwrap().text("1").unwrap();
    el.content().unwrap().text("2").unwrap();
    el.end().unwrap();
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

#[test]
fn test_removed_items() {
    let mut r = Renderer::new();
    let mut el = r.element("div").unwrap();
    el.content().unwrap().text("1").unwrap();
    el.content().unwrap().text("2").unwrap();
    el.end().unwrap();
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

    let mut r = Renderer::from_previous_tree(out.hash_tree);
    let mut el = r.element("div").unwrap();
    el.content().unwrap().text("1").unwrap();
    el.end().unwrap();
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
