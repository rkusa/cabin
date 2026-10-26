use http::HeaderValue;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

/// Everything except printable ASCII (non-ASCII is always encoded), and `%` itself.
const NON_PRINTABLE: &AsciiSet = &CONTROLS.add(b'%');

/// A header value for cabin.js with `value` percent-encoded (UTF-8), as browsers decode header
/// values as latin-1, so non-ASCII UTF-8 bytes would end up garbled. cabin.js decodes it via
/// `decodeURIComponent`.
pub(crate) fn percent_encoded(value: &str) -> HeaderValue {
    HeaderValue::try_from(utf8_percent_encode(value, NON_PRINTABLE).to_string())
        .expect("percent-encoded value is printable ASCII")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encoded() {
        assert_eq!(percent_encoded("Hello World"), "Hello World");
        assert_eq!(
            percent_encoded("100% Grüße 😀\n"),
            "100%25 Gr%C3%BC%C3%9Fe %F0%9F%98%80%0A"
        );
        assert_eq!(percent_encoded(r#"{"a":"ä"}"#), r#"{"a":"%C3%A4"}"#);
    }
}
