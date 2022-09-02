use std::fmt::{self, Write};

pub trait View<A = ()> {
    fn render(self, out: impl Write) -> fmt::Result;
}

impl<V1, V2, A> View<A> for (V1, V2)
where
    V1: View<A>,
    V2: View<A>,
{
    fn render(self, mut out: impl Write) -> fmt::Result {
        self.0.render(&mut out)?;
        self.1.render(&mut out)?;
        Ok(())
    }
}

impl<'a, A> View<A> for &'a str {
    fn render(self, mut out: impl Write) -> fmt::Result {
        out.write_str(self)?;
        Ok(())
    }
}

impl<A> View<A> for String {
    fn render(self, out: impl Write) -> fmt::Result {
        View::<A>::render(self.as_str(), out)
    }
}
