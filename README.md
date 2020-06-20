# async-compression
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

This crate provides adaptors between compression crates and Rust's modern
asynchronous IO types.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Development

When developing you will need to enable appropriate features for the different
test cases to run, the simplest is `cargo test --all-features`, but you can
enable different subsets of features as appropriate for the code you are
testing to avoid compiling all dependencies, e.g. `cargo test --features
gzip,stream`.

## License

Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[1]: https://img.shields.io/crates/v/async-compression.svg?style=flat-square
[2]: https://crates.io/crates/async-compression
[3]: https://img.shields.io/travis/Nemo157/async-compression/master.svg?style=flat-square
[4]: https://travis-ci.com/Nemo157/async-compression
[5]: https://img.shields.io/crates/d/async-compression.svg?style=flat-square
[6]: https://crates.io/crates/async-compression
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/async-compression

[releases]: https://github.com/Nemo157/async-compression/releases
