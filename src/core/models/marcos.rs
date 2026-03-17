use surrealdb::types::*;

// TODO: add support for SurrealValue and work on the base service

#[macro_export]
macro_rules! bitmask_serde {
    ($ty:ident) => {
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i32(self.mask)
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mask = i32::deserialize(deserializer)?;
                Ok($ty { mask })
            }
        }

        impl From<i32> for $ty {
            fn from(mask: i32) -> Self {
                $ty { mask }
            }
        }

        impl From<$ty> for i32 {
            fn from(val: $ty) -> i32 {
                val.mask
            }
        }
    };
}
