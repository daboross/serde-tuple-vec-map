serde-tuple-vec-map
================
[![Build Status][travis-image]][travis-builds]
[![crates.io version badge][cratesio-badge]][cratesio-page]

Deserialize maps or JSON objects in [serde] to a vec of tuples rather than a HashMap for when you're only ever going to iterate over the result.

Usage:

```rust
// replace this:
#[derive(Serialize, Deserialize)]
struct MyStuff {
    data: HashMap<KeyType, ValueType>,
}

// with this:
#[derive(Serialize, Deserialize)]
struct MyStuff {
    #[serde(with = "tuple_vec_map")]
    data: Vec<(KeyType, ValueType)>,
}
```

The serialized format remains exactly the same, the only difference is in how the data is stored.

serde-tuple-vec-map supports no_std builds by use of alloc::Vec. If you're on nightly rust, you can enable this with default-features=false.

Full usage example in `tests/integration.rs`, documentation at https://docs.rs/serde-tuple-vec-map.

[travis-image]: https://travis-ci.org/daboross/serde-tuple-vec-map.svg?branch=master
[travis-builds]: https://travis-ci.org/daboross/serde-tuple-vec-map
[serde]: https://github.com/serde-rs/serde/
[cratesio-badge]: http://meritbadge.herokuapp.com/serde-tuple-vec-map
[cratesio-page]: https://crates.io/crates/serde-tuple-vec-map
