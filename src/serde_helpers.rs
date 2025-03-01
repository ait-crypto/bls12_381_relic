use core::marker::PhantomData;

use pairing::group::GroupEncoding;
use serde::{
    Deserializer, Serializer,
    de::{self, Visitor},
};

struct BytesVisitor<T>(PhantomData<T>);

impl<T> Visitor<'_> for BytesVisitor<T>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(formatter, "a byte-encoded Scalar")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Bytes(v), &self))
    }
}

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: GroupEncoding,
    <T as GroupEncoding>::Repr: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_bytes(value.to_bytes().as_ref())
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> TryFrom<&'a [u8]>,
{
    deserializer.deserialize_bytes(BytesVisitor(PhantomData))
}
