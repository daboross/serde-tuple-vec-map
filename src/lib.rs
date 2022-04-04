//! Deserializing maps to tuple-vecs.
//!
//! To use, just include a `Vec<(String, ...)>` in your struct instead of a `HashMap<String, ...>`
//! and tag it with `#[serde(with = "tuple_vec_map")`:
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
//! a HashMap for the data, the key/value pairs are simply collected into a Vec.
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
}

use core::marker::PhantomData;
use core::{cmp, fmt};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

struct TupleVecMapVisitor<K, V> {
    marker: PhantomData<Vec<(K, V)>>,
}

impl<K, V> TupleVecMapVisitor<K, V> {
    pub fn new() -> Self {
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

/// Serialize an array of `(K, V)` pairs as if it were a `HashMap<K, V>`.
///
/// In formats where dictionaries are ordered, this maintains the input data's order. Each pair is treated as a single
/// entry into the dictionary.
///
/// Behavior when duplicate keys are present in the data is unspecified and serializer-dependent. This function does
/// not check for duplicate keys and will not warn the serializer.
pub fn serialize<K, V, S>(data: &[(K, V)], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize,
    V: Serialize,
{
    serializer.collect_map(data.iter().map(|x| (&x.0, &x.1)))
}

/// Deserialize to a `Vec<(K, V)>` as if it were a `HashMap<K, V>`.
///
/// This directly deserializes into the returned vec with no intermediate allocation.
///
/// In formats where dictionaries are ordered, this maintains the input data's order.
pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<Vec<(K, V)>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    deserializer.deserialize_map(TupleVecMapVisitor::new())
}
