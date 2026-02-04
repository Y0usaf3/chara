#[macro_export]
macro_rules! bitmask_serde {
    ($ty:ident) => {
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u32(self.mask)
            }
        }
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mask = u32::deserialize(deserializer)?;
                Ok($ty { mask })
            }
        }
        impl From<u32> for $ty {
            fn from(mask: u32) -> Self {
                $ty { mask }
            }
        }
    };
}
