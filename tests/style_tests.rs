use cabin::style::collector::StyleCollector;
use cabin::style::{Style as _, SubStyle as _, ThemeExt as _, ThemeSubExt as _};

#[test]
fn basic_properties() {
    let c = StyleCollector::default();
    let c = c.block().p(2).inline_block().px(4);
    insta::assert_snapshot!(c.build());
}

#[test]
fn divide() {
    let mut snapshot = String::new();

    let c = StyleCollector::default();
    let c = c.flex().divide_black().divide_x();
    snapshot += &c.build();

    let c = StyleCollector::default();
    let c = c.divide_x().divide_x_reverse().divide_dashed();
    snapshot += &c.build();

    insta::assert_snapshot!(snapshot);
}

#[test]
fn divide_space_merge() {
    let c = StyleCollector::default();
    let c = c.block().divide_black().divide_y().space_y(1);
    insta::assert_snapshot!(c.build());
}

#[test]
fn pseudo_active() {
    let c = StyleCollector::default();
    let c = c.block().when_active(|s| s.bg_blue_500());
    insta::assert_snapshot!(c.build());
}

#[test]
fn pseudo_combination() {
    let c = StyleCollector::default();
    let c = c
        .when_active(|s| {
            s.bg_blue_500()
                .divide_x()
                .when_focus(|s| s.border_black().divide_x())
        })
        .when_focus(|s| s.border_red_400());
    insta::assert_snapshot!(c.build());
}

#[test]
fn merge_same_modifiers() {
    let c = StyleCollector::default();
    let c = c
        .when_active(|s| s.bg_blue_500())
        .when_active(|s| s.border_red_400());
    insta::assert_snapshot!(c.build());
}

#[test]
fn max_page_width() {
    let c = StyleCollector::default();
    let c = c.max_page_width(1024, |s| s.w(24));
    insta::assert_snapshot!(c.build());
}

#[test]
fn animation() {
    let c = StyleCollector::default();
    let c = c
        .text_gray_700()
        .animate_from(|s| s.bg_none().bg_white().text_gray_700().pointer_events_none())
        .animate_to(|s| {
            s.bg_none()
                .bg_green_500()
                .text_white()
                .pointer_events_none()
        });
    insta::assert_snapshot!(c.build());
}

#[test]
fn skip_empty_base_style() {
    let c = StyleCollector::default().when_hover(|s| s.bg_black());
    let css = c.build();
    assert_eq!(css.matches('{').count(), 1, "{css}");
    assert!(css.contains(":hover"), "{css}");
}

#[test]
fn skip_empty_nested_modifier() {
    let c = StyleCollector::default().when_hover(|s| s.when_focus(|s| s.bg_black()));
    let css = c.build();
    assert_eq!(css.matches('{').count(), 1, "{css}");
    assert!(css.contains(":focus:hover"), "{css}");
}

#[test]
fn keep_animation_only_style() {
    let c = StyleCollector::default().animate_from(|s| s.opacity(0));
    let css = c.build();
    assert!(css.contains("@keyframes"), "{css}");
}

fn render(view: impl cabin::View) -> String {
    match view.render(cabin::render::Renderer::new(false, true)) {
        cabin::view::RenderFuture::Ready(Ok(r)) => r.end().html,
        _ => panic!("view didn't render synchronously"),
    }
}

#[test]
fn element_with_only_empty_styles() {
    use cabin::html::{Common as _, h};

    let html = render(h::div(()).class("x").when_hover(|s| s));
    assert_eq!(html, r#"<div class="x"></div>"#);

    let html = render(h::div(()).when_hover(|s| s));
    assert_eq!(html, "<div></div>");

    let html = render(h::div(()).class("x").when_hover(|s| s.bg_black()));
    assert_eq!(html.matches(' ').count(), 2, "{html}");
}

fn position(css: &str, needle: &str) -> usize {
    css.find(needle)
        .unwrap_or_else(|| panic!("{needle} not found in {css}"))
}

#[test]
fn narrower_max_page_width_comes_later() {
    let c = StyleCollector::default()
        .max_page_width(640, |s| s.w(1))
        .max_page_width(1024, |s| s.w(2));
    let css = c.build();
    assert!(
        position(&css, "max-width: 1024px") < position(&css, "max-width: 640px"),
        "{css}"
    );
}

#[test]
fn narrower_max_container_width_comes_later() {
    let c = StyleCollector::default()
        .max_container_width(640, |s| s.w(1))
        .max_container_width(1024, |s| s.w(2));
    let css = c.build();
    assert!(
        position(&css, "max-width: 1024px") < position(&css, "max-width: 640px"),
        "{css}"
    );
}

#[test]
fn pseudo_class_order() {
    // Declared in reverse, the output order must still follow the fixed variant order.
    let c = StyleCollector::default()
        .when_disabled(|s| s.w(6))
        .when_enabled(|s| s.w(5))
        .when_active(|s| s.w(4))
        .when_focus_visible(|s| s.w(3))
        .when_focus(|s| s.w(2))
        .when_hover(|s| s.w(1));
    let css = c.build();
    let order = [
        ":hover",
        ":focus {",
        ":focus-visible",
        ":active",
        ":enabled",
        ":disabled",
    ]
    .map(|needle| position(&css, needle));
    assert!(order.is_sorted(), "{css}");
}

#[test]
fn pseudo_element_content_is_set_on_pseudo_element() {
    let c = StyleCollector::default().w(1).after(|s| s.w(2));
    let css = c.build();
    assert_eq!(css.matches("content:").count(), 1, "{css}");
    assert!(
        position(&css, "::after") < position(&css, "content:"),
        "{css}"
    );

    let c = StyleCollector::default().before(|s| s.w(2));
    let css = c.build();
    assert_eq!(css.matches('{').count(), 1, "{css}");
    assert!(
        position(&css, "::before") < position(&css, "content:"),
        "{css}"
    );
}

#[test]
fn shorthands_come_before_longhands() {
    let css = StyleCollector::default().border_x().border().build();
    assert!(
        position(&css, "border-width:") < position(&css, "border-inline-width:"),
        "{css}"
    );

    let css = StyleCollector::default()
        .items_center()
        .place_items_center()
        .build();
    assert!(
        position(&css, "place-items:") < position(&css, "align-items:"),
        "{css}"
    );
}

#[test]
fn min_container_width_comes_after_base() {
    let c = StyleCollector::default()
        .min_container_width(640, |s| s.w(1))
        .w(2);
    let css = c.build();
    assert!(
        position(&css, "width: 0.5rem") < position(&css, "@container"),
        "{css}"
    );
}
