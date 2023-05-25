use crate::render::marker::Marker;
use crate::{html, Renderer};

#[tokio::test]
async fn test_server_render_list_identity() {
    let r = Renderer::new();
    let r = r.iter_item(1, html::li("1")).await.unwrap();
    let r = r.iter_item(2, html::li("2")).await.unwrap();
    let r = r.iter_item(3, html::li("3")).await.unwrap();
    let out = r.end();

    assert_eq!(out.view, r#"<li>1</li><li>2</li><li>3</li>"#);

    assert_eq!(
        out.hash_tree,
        vec![
            /*  0 */ Marker::Item(1),
            /*  1 */ Marker::Start, // <li>
            /*  2 */ Marker::Start, // text "1"
            /*  3 */ Marker::End(3068971186),
            /*  4 */ Marker::End(2261384233), // </li>
            /*  5 */ Marker::Item(2),
            /*  6 */ Marker::Start, // <li>
            /*  7 */ Marker::Start, // text "2"
            /*  8 */ Marker::End(205742900),
            /*  9 */ Marker::End(2963935779), // </li>
            /* 10 */ Marker::Item(3),
            /* 11 */ Marker::Start, // <li>
            /* 12 */ Marker::Start, // text "3"
            /* 13 */ Marker::End(2632741828),
            /* 14 */ Marker::End(3332286568), // </li>
            /* 15 */ Marker::End(1982045109)
        ]
        .into()
    );

    // render with 1 and 3 switched
    let r = Renderer::from_previous_tree(out.hash_tree);
    let r = r.iter_item(3, html::li("3")).await.unwrap();
    let r = r.iter_item(2, html::li("2")).await.unwrap();
    let r = r.iter_item(1, html::li("1b")).await.unwrap();
    let out = r.end();

    assert_eq!(out.view, r#"<!--unchanged--><!--unchanged--><li>1b</li>"#);

    assert_eq!(
        out.hash_tree,
        vec![
            Marker::Item(3),
            Marker::Start, // <li>
            Marker::Start, // text "1"
            Marker::End(2632741828),
            Marker::End(3332286568), // </li>
            Marker::Item(2),
            Marker::Start, // <li>
            Marker::Start, // text "2"
            Marker::End(205742900),
            Marker::End(2963935779), // </li>
            Marker::Item(1),
            Marker::Start, // <li>
            Marker::Start, // text "1b"
            Marker::End(93632624),
            Marker::End(3682564943), // </li>
            Marker::End(3710482680)
        ]
        .into()
    );
}
