use core::fmt;
use std::str::FromStr;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const SIZE: usize = 21;
const ALPHABET: [u8; 64] = [
    b'_', b'-', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
    b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
    b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
];

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct NanoId([u8; SIZE]);

impl NanoId {
    pub fn random() -> Self {
        let mask = ALPHABET.len().next_power_of_two() - 1;
        const STEP: usize = 8 * SIZE / 5;
        #[cfg(test)]
        let mut rng = StdRng::seed_from_u64(0);
        #[cfg(not(test))]
        let mut rng = StdRng::from_entropy();
        let mut bytes = [0u8; STEP];
        let mut id = [0u8; SIZE];
        let mut i = 0;

        loop {
            rng.fill(&mut bytes[..]);

            for &byte in &bytes {
                let byte = byte as usize & mask;

                if ALPHABET.len() > byte {
                    id[i] = ALPHABET[byte];
                    i += 1;
                    if i == SIZE {
                        return NanoId(id);
                    }
                }
            }
        }
    }
}

impl fmt::Debug for NanoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NanoId")
            .field(&std::str::from_utf8(&self.0).unwrap())
            .finish()
    }
}

impl fmt::Display for NanoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}

impl FromStr for NanoId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for b in s.as_bytes() {
            match *b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => {}
                b => return Err(ParseError::Byte(b)),
            }
        }
        Ok(NanoId(
            s.as_bytes().try_into().map_err(|_| ParseError::Length)?,
        ))
    }
}

#[derive(Debug)]
pub enum ParseError {
    Byte(u8),
    Length,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Byte(b) => write!(f, "byte `{b}` not in nanoid alphabet"),
            ParseError::Length => f.write_str("nanoid must be exactly 21 characters long"),
        }
    }
}

impl Serialize for NanoId {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(std::str::from_utf8(&self.0).unwrap())
    }
}

mod deserialize {
    use std::fmt;
    use std::str::FromStr;

    use serde::de::{self, Visitor};
    use serde::Deserialize;

    use super::NanoId;

    struct NanoIdVisitor;

    impl<'de> Visitor<'de> for NanoIdVisitor {
        type Value = NanoId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(r#"a string with a fixed length of 21"#)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            NanoId::from_str(v).map_err(de::Error::custom)
        }
    }

    impl<'de> Deserialize<'de> for NanoId {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            d.deserialize_any(NanoIdVisitor)
        }
    }
}
