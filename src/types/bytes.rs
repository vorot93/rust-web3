use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// Raw bytes wrapper
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HexBytes(#[serde(with = "crate::helpers::hex_serialize")] pub bytes::Bytes);

impl<T: Into<bytes::Bytes>> From<T> for HexBytes {
    fn from(data: T) -> Self {
        Self(data.into())
    }
}
