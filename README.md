# Mathio 🚀️ 

A ***lightweight***, ***simple*** and ***straight*** opinionated library for **math** used in other projects.

Current features are:

* Fractions


### Examples:

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
