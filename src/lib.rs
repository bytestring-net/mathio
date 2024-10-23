#![doc = include_str!("../README.md")]

mod fractions;
mod period;

pub use crate::fractions::*;
pub use crate::period::*;


#[test]
fn test() {
    let a = Frac32::ONE * Frac32::new(1, 2);
    assert_eq!(a, Frac32::new(1, 2));
    println!("{a}");

    let a = Frac32::new(1, 6) + Frac32::new(1, 4);
    assert_eq!(a, Frac32::new(5, 12));
    println!("{a}");
}