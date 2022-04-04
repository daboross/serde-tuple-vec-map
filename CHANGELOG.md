Unreleased
==========


1.0.1 (2022-04-04)
==================

- Fix typo in vec initialization code which limited Vec pre-allocation
  size to 4069, rather than 4096 as was intended.


1.0.0 (2019-02-08)
==================

- Add support for serializing from (but not deserializing to) `&[(K, V)]`
- Specify more exact behavior in documentation
- Migrate from git tag messages to an explicit CHANGELOG.md file


0.2.2 (2019-02-08)
==================

- Remove reliance on nightly rust, following the stabilization of the
  'alloc' library.

  When using default-features = false, this crate now no longer depends
  on nightly, but instead on Rust 1.36.0+.

  As the prior dependency was on an unstable nightly feature, I don't
  consider this a breaking change.
- Update CI configurations to specify and test the minimum suppored rust
  version of 1.13.0 when default-features is enabled.


0.2.1 (2018-01-04)
==================

- Update documentation links to point to docs.rs
- Clean up documentation, make it more informative and less like a
  sales pitch.


0.2.0 (2017-04-26)
==================

- Update from serde 0.9 to serde 1.0 stable


0.1.0 (2017-03-28)
==================

- Initial release! Fully working.
