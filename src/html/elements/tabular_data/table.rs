use std::fmt;
use std::marker::PhantomData;

use crate::html::attributes::Attributes;
use crate::html::elements::scripting::script::Script;
use crate::html::template::Template;
use crate::html::{create, AddChild, Html};
use crate::render::ElementRenderer;
use crate::view::{Pair, View};

pub fn table() -> Html<(), (), Table<()>> {
    create("table", Table(PhantomData), ())
}

pub fn caption() -> Html<(), (), Caption> {
    create("caption", Caption(()), ())
}

pub fn colgroup() -> Html<(), (), Colgroup> {
    create("colgroup", Colgroup(()), ())
}

pub fn thead() -> Html<(), (), THead> {
    create("thead", THead(()), ())
}

pub fn tbody() -> Html<(), (), TBody> {
    create("tbody", TBody(()), ())
}

pub fn tr() -> Html<(), (), Tr> {
    create("tr", Tr(()), ())
}

pub fn tfoot() -> Html<(), (), TFoot> {
    create("tfoot", TFoot(()), ())
}

pub struct Table<Prev>(PhantomData<Prev>);
impl<Prev> Attributes for Table<Prev> {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct Caption(());
impl Attributes for Caption {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct Colgroup(());
impl Attributes for Colgroup {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct THead(());
impl Attributes for THead {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct TBody(());
impl Attributes for TBody {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct Tr(());
impl Attributes for Tr {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub struct TFoot(());
impl Attributes for TFoot {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

#[macro_export]
macro_rules! impl_add_child {
    ($parent:ty, $child:ty, $result:ty) => {
        impl<LV, LA, RV, RA> AddChild<Html<RV, RA, $child>> for Html<LV, LA, $parent>
        where
            LV: View + Send + 'static,
            LA: Attributes + Send + 'static,
            RV: View + Send + 'static,
            RA: Attributes + Send + 'static,
        {
            type Output = Html<Pair<LV, Html<RV, RA, $child>>, LA, $result>;

            fn add_child(self, child: Html<RV, RA, $child>) -> Self::Output {
                Html {
                    tag: self.tag,
                    attrs: self.attrs,
                    on_click: self.on_click,
                    kind: Table(PhantomData),
                    content: Pair::new(self.content, child),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_add_child_anywhere {
    ($parent:ty, $child:ty, $result:ty) => {
        impl<T, LV, LA, RV, RA> AddChild<Html<RV, RA, $child>> for Html<LV, LA, $parent>
        where
            LV: View + Send + 'static,
            LA: Attributes + Send + 'static,
            RV: View + Send + 'static,
            RA: Attributes + Send + 'static,
        {
            type Output = Html<Pair<LV, Html<RV, RA, $child>>, LA, $result>;

            fn add_child(self, child: Html<RV, RA, $child>) -> Self::Output {
                Html {
                    tag: self.tag,
                    attrs: self.attrs,
                    on_click: self.on_click,
                    kind: self.kind,
                    content: Pair::new(self.content, child),
                }
            }
        }
    };
}

macro_rules! impl_add_unit {
    ($parent:ident) => {
        impl<T, LV, LA> AddChild<()> for Html<LV, LA, $parent<T>>
        where
            LV: View + Send + 'static,
            LA: Attributes + Send + 'static,
        {
            type Output = Self;

            fn add_child(self, _child: ()) -> Self::Output {
                self
            }
        }
    };
}

impl_add_unit!(Table);
impl_add_child_anywhere!(Table<T>, Script, Table<T>);
impl_add_child_anywhere!(Table<T>, Template, Table<T>);

impl_add_child!(Table<()>, Caption, Table<Caption>);
impl_add_child!(Table<()>, Colgroup, Table<Colgroup>);
impl_add_child!(Table<()>, THead, Table<THead>);
impl_add_child!(Table<()>, TBody, Table<TBody>);
impl_add_child!(Table<()>, Tr, Table<Tr>);
impl_add_child!(Table<()>, TFoot, Table<TFoot>);

impl_add_child!(Table<Caption>, Colgroup, Table<Colgroup>);
impl_add_child!(Table<Caption>, THead, Table<THead>);
impl_add_child!(Table<Caption>, TBody, Table<TBody>);
impl_add_child!(Table<Caption>, Tr, Table<Tr>);
impl_add_child!(Table<Caption>, TFoot, Table<TFoot>);

impl_add_child!(Table<Colgroup>, Colgroup, Table<Colgroup>);
impl_add_child!(Table<Colgroup>, THead, Table<THead>);
impl_add_child!(Table<Colgroup>, TBody, Table<TBody>);
impl_add_child!(Table<Colgroup>, Tr, Table<Tr>);
impl_add_child!(Table<Colgroup>, TFoot, Table<TFoot>);

impl_add_child!(Table<THead>, TBody, Table<TBody>);
impl_add_child!(Table<THead>, Tr, Table<Tr>);
impl_add_child!(Table<THead>, TFoot, Table<TFoot>);

impl_add_child!(Table<TBody>, TBody, Table<TBody>);
impl_add_child!(Table<TBody>, TFoot, Table<TFoot>);

impl_add_child!(Table<Tr>, Tr, Table<Tr>);
impl_add_child!(Table<Tr>, TFoot, Table<TFoot>);

#[test]
fn table_valid_children() {
    use crate::html;

    // caption? -> colgroup* -> thead? -> (tbody* | tr*) -> tfoot?
    // template/script in between

    html::table().add_child(());

    html::table()
        .add_child(html::caption())
        .add_child(())
        .add_child(html::script())
        .add_child(html::template())
        .add_child(html::colgroup())
        .add_child(html::colgroup())
        .add_child(())
        .add_child(html::script())
        .add_child(html::template())
        .add_child(html::thead())
        .add_child(())
        .add_child(html::script())
        .add_child(html::template())
        .add_child(html::tbody())
        .add_child(html::tbody())
        .add_child(())
        .add_child(html::script())
        .add_child(html::template())
        .add_child(html::tfoot())
        .add_child(())
        .add_child(html::script())
        .add_child(html::template());

    html::table().add_child(html::colgroup());
    html::table()
        .add_child(html::thead())
        .add_child(html::tr())
        .add_child(html::tr());
    html::table().add_child(html::tbody());
    html::table().add_child(html::tr());
    html::table().add_child(html::tfoot());
    html::table().add_child(html::template());
    html::table().add_child(html::script());
}
