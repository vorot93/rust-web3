use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// Raw bytes wrapper
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HexBytes(pub bytes::Bytes);

impl<T: Into<bytes::Bytes>> From<T> for HexBytes {
    fn from(data: T) -> Self {
        Self(data.into())
    }
}

impl Serialize for HexBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialized = String::with_capacity(2 + self.0.len() * 2);
        serialized.push_str("0x");
        serialized.push_str(&hex::encode(&self.0));
        serializer.serialize_str(serialized.as_ref())
    }
}

impl<'a> Deserialize<'a> for HexBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_identifier(HexBytesVisitor)
    }
}

struct HexBytesVisitor;

impl<'a> Visitor<'a> for HexBytesVisitor {
    type Value = HexBytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value.len() >= 2 && &value[0..2] == "0x" {
            let bytes = hex::decode(&value[2..]).map_err(|e| Error::custom(format!("Invalid hex: {}", e)))?;
            Ok(Bytes(bytes))
        } else {
            Err(Error::invalid_value(Unexpected::Str(value), &"0x prefix"))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(value.as_ref())
    }
}
