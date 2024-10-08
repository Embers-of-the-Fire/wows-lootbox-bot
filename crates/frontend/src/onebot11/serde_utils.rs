use core::fmt;

use serde::{de::Visitor, Deserializer};

pub fn deserialize_onebot_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct MaxVisitor;

    impl<'de> Visitor<'de> for MaxVisitor {
        /// Return type of this visitor. This visitor computes the max of a
        /// sequence of values of type T, so the type of the maximum is T.
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a onebot-11 styled boolean value")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid boolean value: {}",
                    v
                ))),
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid boolean value: {}",
                    v
                ))),
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                "yes" | "true" => Ok(true),
                "no" | "false" => Ok(false),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid boolean value: {}",
                    v
                ))),
            }
        }
    }

    let visitor = MaxVisitor;
    deserializer.deserialize_any(visitor)
}
