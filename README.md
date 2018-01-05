serde-tuple-vec-map
================
[![Build Status][travis-image]][travis-builds]
[![crates.io version badge][cratesio-badge]][cratesio-page]

Deserialize maps or JSON objects in [serde] to a vec of tuples rather than a HashMap for when you're only ever going to iterate over the result.

Don't waste space and time making a whole HashMap when you will never use it!

To use, instead of:

```rust
#[derive(Serialize, Deserialize)]
struct MyStuff {
    data: HashMap<String, ValueType>,
}

```

You can write:

```rust
#[derive(Serialize, Deserialize)]
struct MyStuff {
    #[serde(with = "tuple_vec_map")]
    data: Vec<(String, ValueType)>,
}
```

Similar to [serde], serde-tuple-vec-map supports the use of `no_std` with `collections::Vec`.
To enable this, simply depend on `serde-tuple-vec-map` with `default-features = false`.

Full usage example in `tests/integration.rs`, documentation at https://docs.rs/serde-tuple-vec-map.

[travis-image]: https://travis-ci.org/daboross/serde-tuple-vec-map.svg?branch=master
[travis-builds]: https://travis-ci.org/daboross/serde-tuple-vec-map
[serde]: https://github.com/serde-rs/serde/
[cratesio-badge]: http://meritbadge.herokuapp.com/serde-tuple-vec-map
[cratesio-crate]: https://crates.io/crates/serde-tuple-vec-map
