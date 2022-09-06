use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

pub struct Attribute<N> {
    name: &'static str,
    value: Cow<'static, str>,
    next: N,
}

pub trait Attributes {
    fn hash(&self, hasher: impl Hasher);
    fn render(self, out: impl Write) -> Result<(), fmt::Error>;
}

impl Attributes for () {
    fn hash(&self, _hasher: impl Hasher) {}

    fn render(self, _out: impl Write) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl<N> Attribute<N> {
    pub fn new(name: &'static str, value: impl Into<Cow<'static, str>>, next: N) -> Self {
        Self {
            name,
            value: value.into(),
            next,
        }
    }
}

impl<N> Attributes for Attribute<N>
where
    N: Attributes,
{
    fn hash(&self, mut hasher: impl Hasher) {
        hasher.write(self.name.as_bytes());
        hasher.write(self.value.as_bytes());

        self.next.hash(hasher)
    }

    fn render(self, mut out: impl Write) -> Result<(), fmt::Error> {
        write!(
            &mut out,
            r#" {}="{}""#,
            self.name, // TODO: validate/escape attr name
            escape_attribute_value(&self.value)
        )?;

        self.next.render(out)
    }
}

pub fn escape_attribute_value(input: &str) -> Cow<str> {
    let mut replacements = input
        .char_indices()
        .filter_map(|(i, ch)| escape_attribute_value_char(ch).map(|s| (i, s)))
        .peekable();
    if replacements.peek().is_none() {
        return Cow::Borrowed(input);
    }

    let mut escaped = String::with_capacity(input.len());
    let mut pos = 0;
    for (i, sub) in replacements {
        if i > pos {
            escaped.push_str(&input[pos..i]);
        }
        escaped.push_str(sub);
        pos = i + 1;
    }
    if pos < input.len() {
        escaped.push_str(&input[pos..input.len()]);
    }

    Cow::Owned(escaped)
}

fn escape_attribute_value_char(ch: char) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '\'' => Some("&apos;"),
        '&' => Some("&amp;"),
        '"' => Some("&quot;"),
        _ => None,
    }
}
