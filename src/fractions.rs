use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}, fmt::{Display, self}};
use core::cmp::{PartialEq, PartialOrd, Ordering, Eq};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Fraction i32 type
#[derive(Clone, Copy, Debug)]
#[derive(Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Frac32 {
    pub numerator: i32,
    pub denominator: i32,
}
impl Frac32 {
    pub const ONE: Self = Self::new(1,1);
    /// Creates new fraction
    pub const fn new(num: i32, den: i32) -> Frac32 {
        Frac32 { numerator: num, denominator: den }
    }
    /// Will evaluate the fraction and return a f32 approximation
    pub fn eval(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub fn elem(self) -> Self {
        let common = highest_divisor(self.numerator, self.denominator);
        Self::new(self.numerator/common, self.denominator/common)
    }
}
impl Display for Frac32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{})", self.numerator, self.denominator)
    }
}

impl Add for Frac32 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Frac32::new(
            self.numerator*other.denominator + other.numerator*self.denominator,
            self.denominator * other.denominator,
        ).elem()
    }
}
impl AddAssign for Frac32 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Sub for Frac32 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Frac32::new(
            self.numerator*other.denominator - other.numerator*self.denominator,
            self.denominator * other.denominator,
        ).elem()
    }
}
impl SubAssign for Frac32 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl Mul for Frac32 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Frac32::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        ).elem()
    }
}
impl MulAssign for Frac32 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl Div for Frac32 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Frac32::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        ).elem()
    }
}
impl DivAssign for Frac32 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl PartialEq for Frac32 {
    fn eq(&self, other: &Self) -> bool {
        let a = self.numerator*other.denominator;
        let b = other.numerator*self.denominator;
        a == b
    }
}
impl PartialOrd for Frac32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Frac32 {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.numerator*other.denominator;
        let b = other.numerator*self.denominator;
        a.cmp(&b)
    }
}

/// Find the highest divisor
pub fn highest_divisor(a: i32, b: i32) -> i32 {
    let mut divisor = 1;
    let mut max = a.max(b);
    let mut min = a.min(b);

    let mut i = 1;
    while i <= min {
        if min%i == 0 && max%i == 0 && i != 1 {
            divisor *= i;
            min /= i;
            max /= i;
        } else {
            i += 1;
        }
    }

    divisor
}

/// Find the lowest multiple
pub fn lowest_multiple(a: i32, b: i32) -> i32 {
    a*b / highest_divisor(a, b)
}