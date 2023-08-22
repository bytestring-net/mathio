//! A ***lightweight***, ***simple*** and ***straight-forward*** opinionated library for **math** used in other projects.
//! 
//! Current features are:
//! 
//! * Fractions
//! * Tweening
//! * Periods
//! 
//! 
//! ### === Examples ===
//! 
//! **Fractions**
//! 
//! Great for expressing lossless floats
//! ```
//! use mathio::Fraction;
//! 
//! let frac_1 = Fraction::new(2, 3);
//! let frac_2 = Fraction::new(3, 6);
//!  
//! assert_eq!(frac_1 + frac_2, Fraction::new(7, 6));
//! assert_eq!(frac_1 - frac_2, Fraction::new(1, 6));
//! assert_eq!(frac_1 * frac_2, Fraction::new(1, 3));
//! assert_eq!(frac_1 / frac_2, Fraction::new(4, 3));
//! assert_eq!(frac_1 >= frac_2, true);
//! ```
//! 
//! **Periods**
//! 
//! Good for clamping repeating values (angles, sin/cosine, etc.)
//! ```
//! let period = 360.0;
//! assert_eq!(315.0, periodical(period, -45.0));
//! assert_eq!(45.0, periodical(period, 45.0));
//! assert_eq!(0.0, periodical(period, 360.0));
//! assert_eq!(90.0, periodical(period, 450.0));
//! ```
//! 
//! ### === Licensing ===
//! Released under both APACHE and MIT licenses. Pick one that suits you the most!



mod fractions;
mod period;
mod tween;

pub use crate::fractions::*;
pub use crate::period::*;
pub use crate::tween::*;