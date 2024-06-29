use core::marker::PhantomData;

use generic_array::{ArrayLength, GenericArray};
use pairing::group::GroupEncoding;
use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};

struct BytesVisitor<T>(PhantomData<T>);

impl<'de, T, N> Visitor<'de> for BytesVisitor<T>
where
    N: ArrayLength,
    T: GroupEncoding<Repr = GenericArray<u8, N>>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(formatter, "a byte-encoded Scalar")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = v
            .try_into()
            .map_err(|_| E::invalid_length(v.len(), &self))?;
        let value = T::from_bytes(bytes);
        if value.is_some().unwrap_u8() == 1 {
            Ok(value.unwrap())
        } else {
            Err(E::invalid_value(
                de::Unexpected::Bytes(bytes.as_ref()),
                &self,
            ))
        }
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

pub fn deserialize<'de, D, T, N>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    N: ArrayLength,
    T: GroupEncoding<Repr = GenericArray<u8, N>>,
{
    deserializer.deserialize_bytes(BytesVisitor(PhantomData))
}
