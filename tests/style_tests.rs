use cabin::style::collector::StyleCollector;
use cabin::style::{Style as _, ThemeExt as _};

#[test]
fn basic_properties() {
    let c = StyleCollector::default();
    let c = c.block().p(2).inline_block().px(4);
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn divide() {
    let mut snapshot = String::new();

    let c = StyleCollector::default();
    let c = c.flex().divide_black().divide_x();
    snapshot += &c.build(false).unwrap();

    let c = StyleCollector::default();
    let c = c.divide_x().divide_x_reverse().divide_dashed();
    snapshot += &c.build(false).unwrap();

    insta::assert_snapshot!(snapshot);
}

#[test]
fn divide_space_merge() {
    let c = StyleCollector::default();
    let c = c.block().divide_black().divide_y().space_y();
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn pseudo_active() {
    let c = StyleCollector::default();
    let c = c.block().active(|s| s.bg_blue_500());
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn pseudo_combination() {
    let c = StyleCollector::default();
    let c = c
        .active(|s| {
            s.bg_blue_500()
                .divide_x()
                .focus(|s| s.border_black().divide_x())
        })
        .focus(|s| s.border_red_400());
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn merge_same_modifiers() {
    let c = StyleCollector::default();
    let c = c.active(|s| s.bg_blue_500()).active(|s| s.border_red_400());
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn max_page_width() {
    let c = StyleCollector::default();
    let c = c.max_page_width(1024, |s| s.w(24));
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn animation() {
    use cabin::prelude::*;
    use cabin::tailwind::registry::StyleRegistry;

    let mut r = StyleRegistry::default();
    tw![
        tw::text::GRAY_700,
        (
            tw::bg::NONE,
            tw::bg::WHITE,
            tw::text::GRAY_700,
            tw::pointer_events::NONE
        )
            .animate_from(),
        (
            tw::bg::NONE,
            tw::bg::GREEN_500,
            tw::text::WHITE,
            tw::pointer_events::NONE
        )
            .animate_to()
    ]
    .append_to(&mut r);
    insta::assert_snapshot!(r.build(false));
}
