use cabin::style::collector::StyleCollector;
use cabin::style::{Style as _, SubStyle as _, ThemeExt as _, ThemeSubExt as _};

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
    let c = c.block().when_active(|s| s.bg_blue_500());
    insta::assert_snapshot!(c.build(false).unwrap());
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
    insta::assert_snapshot!(c.build(false).unwrap());
}

#[test]
fn merge_same_modifiers() {
    let c = StyleCollector::default();
    let c = c
        .when_active(|s| s.bg_blue_500())
        .when_active(|s| s.border_red_400());
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
    insta::assert_snapshot!(c.build(false).unwrap());
}
