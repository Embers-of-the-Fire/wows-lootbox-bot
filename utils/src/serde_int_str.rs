use core::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T: FromStr>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + FromStr,
    {
        /// Return type of this visitor. This visitor computes the max of a
        /// sequence of values of type T, so the type of the maximum is T.
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a integer-valued string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse()
                .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self))
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_str(visitor)
}

pub fn serialize_to_str<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    serializer.serialize_str(value.to_string().as_str())
}
