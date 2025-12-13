use cabin::prelude::*;
use cabin::tailwind::registry::StyleRegistry;

#[test]
fn basic_properties() {
    let mut r = StyleRegistry::default();
    tw![tw::BLOCK, tw::p(2), tw::INLINE_BLOCK, tw::px(4)].append_to(&mut r);
    insta::assert_snapshot!(r.build(false));
}

#[test]
fn divide() {
    let mut r = StyleRegistry::default();
    tw![tw::FLEX, tw::divide::BLACK, tw::divide::x(1)].append_to(&mut r);
    insta::assert_snapshot!(r.build(false));
}
