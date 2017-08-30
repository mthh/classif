# classif
[![Build Status](https://travis-ci.org/mthh/classif.svg?branch=master)](https://travis-ci.org/mthh/classif)


Rust library for data classification, especially methods used in cartography, and simple statistics.  
Availables classification methods: **Jenks Natural Breaks**, **Equal Invervals**, **Quantiles**, **Arithmetic Progression** and **Head-Tail Breaks**.
Statistical functions: *mean, median, kurtosis, variance, standard deviation, root mean square, harmonic mean* and *geometric mean*.

## Usage

To use `classif`, first add this to your `Cargo.toml`:

```toml
[dependencies]
classif = "0.0.2"
```

Then, add this to your crate root:

```rust
extern crate classif;
```

The API documentation of this library can be found at [https://docs.rs/classif](https://docs.rs/classif).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
