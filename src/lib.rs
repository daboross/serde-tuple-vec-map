//! Deserializing maps to tuple-vecs.
//!
//! To use, just include a [`Vec<(String, ...)>`][Vec] in your struct instead of a
//! [`HashMap<String, ...>`][std::collections::HashMap] and tag it with `#[serde(with = "tuple_vec_map")`:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use] extern crate serde_derive;
//! extern crate tuple_vec_map;
//!
//! #[derive(Serialize, Deserialize)]
//! struct SomeData {
//!     other_stuff: u32,
//!     #[serde(with = "tuple_vec_map")]
//!     inner_data: Vec<(String, String)>,
//! }
//! # fn main() {}
//! ```
//!
//! That's it! Now your structure accepts an inner_data Map or JSON Object, and instead of making
//! a [`HashMap`][std::collections::HashMap] for the data, the key/value pairs are simply collected into a [`vec`].
//!
//! ## Features
//!
//! To use without `std`, depend on `serde-tuple-vec-map` with `default-features = false`. This will still
//! depend on the `alloc` crate, and requires Rust 1.36.0 or newer.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/serde-tuple-vec-map/1.0.1")]

extern crate serde;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
mod core {
    // this mirrors serde's setup for std/non-std.
    pub use std::cmp;
    pub use std::fmt;
    pub use std::marker;
    pub use std::ops;
}

use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Deref;
use core::{cmp, fmt};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

struct TupleVecMapVisitor<K, V> {
    marker: PhantomData<Vec<(K, V)>>,
}

impl<K, V> TupleVecMapVisitor<K, V> {
    fn new() -> Self {
        TupleVecMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for TupleVecMapVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = Vec<(K, V)>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Vec<(K, V)>, E> {
        Ok(Vec::new())
    }

    #[inline]
    fn visit_map<T>(self, mut access: T) -> Result<Vec<(K, V)>, T::Error>
    where
        T: MapAccess<'de>,
    {
        let mut values = Vec::with_capacity(cmp::min(access.size_hint().unwrap_or(0), 4096));

        while let Some((key, value)) = access.next_entry()? {
            values.push((key, value));
        }

        Ok(values)
    }
}

/// Serialize a [`slice`] of `(K, V)` pairs as if it were a [`HashMap<K, V>`][std::collections::HashMap].
///
/// In formats where dictionaries are ordered, this maintains the input data's order. Each pair is treated as a single
/// entry into the dictionary.
///
/// Behavior when duplicate keys are present in the data is unspecified and serializer-dependent. This function does
/// not check for duplicate keys and will not warn the serializer.
///
/// # Errors
///
/// Errors if and only if the given serializer emits an error.
pub fn serialize<K, V, S>(data: &[(K, V)], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize,
    V: Serialize,
{
    serializer.collect_map(data.iter().map(|x| (&x.0, &x.1)))
}

/// Deserialize to a [`Vec<(K, V)>`] as if it were a [`HashMap<K, V>`][std::collections::HashMap].
///
/// This directly deserializes into the returned vec with no intermediate allocation.
///
/// In formats where dictionaries are ordered, this maintains the input data's order.
///
/// # Errors
///
/// Errors if and only if the given deserializer emits an error. Note, this may occur if using this function to
/// deserialize data that the serializer can't treat as a map.
pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<Vec<(K, V)>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    deserializer.deserialize_map(TupleVecMapVisitor::new())
}

/// Vec-as-map serialization wrapper.
///
/// This is intended when `tuple_vec_map` behavior is required for the outermost serialized or deserialized object.
///
/// While [`Wrapper`] can be constructed with any inner value, it only implements useful traits when the inner value is
/// either `Vec<(K, V)>` or `&[(K, V)]` for some `K` and `V`. Thus, utility methods [`Wrapper::from_slice`] and
/// [`Wrapper::from_vec`] have been created for convenient type-error-free construction.
///
/// # Serialization
///
/// When `T` can be treated as `&[(K, V)]`, `Wrapper<T>` implements [`Serialize`], serializing the inner value as if it
/// were a [`HashMap`][std::collections::HashMap].
///
/// As an example, this works for `Wrapper<&[(K, V)]>`, `Wrapper<Vec<(K, V)>>`, `Wrapper<[(K, V); N]>`, as well as
/// other Vec-like types, such as `Wrapper<SmallVec<[(K, V); 4]>>` using
/// [`SmallVec`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html) from the
/// [`smallvec`](https://docs.rs/smallvec/) crate.
///
/// In formats where dictionaries are ordered, this maintains the input data's order. Each pair is treated as a single
/// entry into the dictionary.
///
/// Behavior when duplicate keys are present in the data is unspecified and serializer-dependent. This function does
/// not check for duplicate keys and will not warn the serializer.
///
/// ## Example
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = [("hello", "world"), ("answer", "42")];
/// let out = serde_json::to_string(&tuple_vec_map::Wrapper::from_slice(&data))?;
/// assert_eq!(out, r#"{"hello":"world","answer":"42"}"#);
/// # Ok(()) }
/// ```
///
/// # Deserialization
///
/// `Wrapper<Vec<(K, V)>>` implements [`Deserialize`], deserializing from a map as if it were a
/// [`HashMap<K, V>`][std::collections::HashMap].
///
/// This directly deserializes into the wrapped vec with no intermediate allocation.
///
/// In formats where dictionaries are ordered, this maintains the input data's order.
///
/// ## Example
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = r#"{"hello": "world", "answer": "42"}"#;
/// let out: tuple_vec_map::Wrapper<Vec<(String, String)>> = serde_json::from_str(data)?;
/// assert_eq!(
///     out.into_inner(),
///     vec![
///         ("hello".to_owned(), "world".to_owned()),
///         ("answer".to_owned(), "42".to_owned()),
///     ],
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Copy)]
pub struct Wrapper<T>(
    /// The inner value, either to be serialized or freshly deserialized.
    pub T,
);

impl<'a, K, V> Wrapper<&'a [(K, V)]> {
    /// Creates a wrapper from the given slice.
    pub fn from_slice(slice: &'a [(K, V)]) -> Self {
        Wrapper(slice)
    }
}

impl<K, V> Wrapper<Vec<(K, V)>> {
    /// Creates a wrapper from the given [`Vec`].
    #[must_use]
    pub fn from_vec(vec: Vec<(K, V)>) -> Self {
        Wrapper(vec)
    }
}

impl<T> Wrapper<T> {
    /// Takes the inner value out of this [`Wrapper`].
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Debug> Debug for Wrapper<T> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, T, K, V> Serialize for Wrapper<T>
where
    T: Deref<Target = [(K, V)]>,
    K: 'a + Serialize,
    V: 'a + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.0.iter().map(|x| (&x.0, &x.1)))
    }
}

impl<'de, K, V> Deserialize<'de> for Wrapper<Vec<(K, V)>>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_map(TupleVecMapVisitor::new())
            .map(Wrapper)
    }
}
