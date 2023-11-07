//! Contains a `serde` serializable pointer type.

use serde::de::Error;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

#[derive(serde::Deserialize, Clone, Copy, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NonNullPtr<T = u8>(#[serde(deserialize_with = "from_hex")] pub NonNull<T>);

impl<T> NonNullPtr<T> {
    pub fn new(inner: usize) -> Option<Self> {
        NonNull::new(inner as *mut T).map(NonNullPtr)
    }
}

impl<T> serde::Serialize for NonNullPtr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{:#X}", (self.0.as_ptr() as usize)).serialize(serializer)
    }
}

impl<T> Debug for NonNullPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NonNullPtr")
            .field(&format_args!("{:#X}", self.0.as_ptr() as usize))
            .finish()
    }
}

impl<T> Deref for NonNullPtr<T> {
    type Target = NonNull<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NonNullPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<usize> for NonNullPtr<T> {
    fn from(value: usize) -> Self {
        Self(NonNull::new(value as *mut T).expect("Passed null pointer"))
    }
}

fn from_hex<'de, D, T>(deserializer: D) -> Result<NonNull<T>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let value = usize::from_str_radix(&s[2..], 16).map_err(D::Error::custom)?;

    NonNull::new(value as *mut T).ok_or_else(|| D::Error::custom("Invalid pointer"))
}
