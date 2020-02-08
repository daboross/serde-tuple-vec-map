#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate serde;
extern crate tuple_vec_map;

use serde::Deserialize;

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestData {
    #[serde(with = "tuple_vec_map")]
    score: Vec<(String, u32)>,
}

#[test]
fn test_deserialization() {
    let data = json!({
        "score": {
            "user1": 50,
            "user2": 30,
            "user3": 0,
            "user4": 100,
        }
    });

    let deserialized = TestData::deserialize(data).expect("expected successful deserialization");

    // Note: This also tests that order is maintained.
    assert_eq!(
        deserialized,
        TestData {
            score: vec![
                ("user1".to_owned(), 50),
                ("user2".to_owned(), 30),
                ("user3".to_owned(), 0),
                ("user4".to_owned(), 100),
            ],
        }
    )
}

#[test]
fn test_serialization() {
    let data = TestData {
        score: vec![
            ("a_guy".to_owned(), 200),
            ("b_guy".to_owned(), 300),
            ("c_guy".to_owned(), 400),
        ],
    };

    let serialized = serde_json::to_value(data).expect("expected data to serialize successfully");

    assert_eq!(
        serialized,
        json!({
            "score": {
                "a_guy": 200,
                "b_guy": 300,
                "c_guy": 400,
            }
        })
    );
}

#[test]
fn serializing_from_slice_works() {
    #[derive(Serialize)]
    struct Data<'a> {
        #[serde(with = "tuple_vec_map")]
        inner: &'a [(&'a str, &'a str)],
    }

    let data = Data { inner: &[("answer", "fourty-two")] };

    let ser = serde_json::to_value(data).unwrap();
    assert_eq!(ser, json!({
        "inner": {
            "answer": "fourty-two",
        }
    }));
}
