#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(collections))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate collections;

extern crate serde;

extern crate tuple_vec_map;

use serde::Deserialize;

#[cfg(not(feature = "std"))]
use collections::{String, Vec};
#[cfg(not(feature = "std"))]
use collections::borrow::ToOwned;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestData {
    #[serde(with = "tuple_vec_map")]
    score: Vec<(String, u32)>,
}

#[test]
fn deserialize_some_data() {
    let data = json!({
        "score": {
            "user1": 50,
            "user2": 30,
            "user3": 0,
            "user4": 100,
        }
    });

    let deserialized = TestData::deserialize(data).expect("expected successful deserialization");

    // Note: This also tests that the data is kept in the same order as in the JSON.
    // I *think* that's what we want to do...
    assert_eq!(deserialized, TestData {
        score: vec![
            ("user1".to_owned(), 50),
            ("user2".to_owned(), 30),
            ("user3".to_owned(), 0),
            ("user4".to_owned(), 100),
        ],
    })
}

#[test]
fn serialize_it_back_up() {
    let data = TestData {
        score: vec![
            ("a_guy".to_owned(), 200),
            ("b_guy".to_owned(), 300),
            ("c_guy".to_owned(), 400),
        ],
    };

    let serialized = serde_json::to_value(data).expect("expected data to serialize successfully");

    assert_eq!(serialized, json!({
        "score": {
            "a_guy": 200,
            "b_guy": 300,
            "c_guy": 400,
        }
    }));
}
