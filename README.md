# Mathio 🚀️ 

[![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-blue.svg?label=license)](./LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/mathio.svg)](https://crates.io/crates/mathio)
[![Released API docs](https://docs.rs/mathio/badge.svg)](https://docs.rs/mathio)

A ***lightweight***, ***simple*** and ***straight-forward*** opinionated library for **math** used in other projects.

Current features are:

* Fractions


### === Examples ===
```rust
use mathio::Fraction;

let frac_1 = Fraction::new(2, 3);
let frac_2 = Fraction::new(3, 6);
 
assert_eq!(frac_1 + frac_2, Fraction::new(7, 6));
assert_eq!(frac_1 - frac_2, Fraction::new(1, 6));
assert_eq!(frac_1 * frac_2, Fraction::new(1, 3));
assert_eq!(frac_1 / frac_2, Fraction::new(4, 3));
assert_eq!(frac_1 >= frac_2, true);
```

### === Licensing ===
Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses. Pick one that suits you the most!
