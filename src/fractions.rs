use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}, fmt::{Display, self}};
use core::cmp::{PartialEq, PartialOrd, Ordering, Eq};

const PRIMES: [i32; 100] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541];

/// ## Fraction
/// Allows you to represent float in a lossless way.
/// 
/// ### Examples:
/// ```
/// use mathio::Fraction;
/// 
/// let frac_1 = Fraction::new(2, 3);
/// let frac_2 = Fraction::new(3, 6);
/// 
/// assert_eq!(frac_1 + frac_2, Fraction::new(7, 6));
/// assert_eq!(frac_1 - frac_2, Fraction::new(1, 6));
/// assert_eq!(frac_1 * frac_2, Fraction::new(1, 3));
/// assert_eq!(frac_1 / frac_2, Fraction::new(4, 3));
/// assert_eq!(frac_1 >= frac_2, true);
/// 
/// ```
#[derive(Debug, Eq, Copy, Clone)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}
impl Fraction {
    /// Creates new fraction.
    /// ```
    /// use mathio::Fraction;
    /// 
    /// let frac = Fraction::new(1, 2); // 1/2 or one_half
    /// ```
    pub fn new (num: i32, den: i32) -> Fraction {
        Fraction { numerator: num, denominator: den }
    }
    /// Will evaluate the fraction and return a f32 approximation.
    pub fn eval (&self) -> f32 {
        self.numerator as f32/self.denominator as f32
    }
    /// Will check and simplify the fraction. Checks up to the first 100 prime numbers.
    /// If your fraction is a multiplication of a higher prime numbers, simplification will not work.
    pub fn simplify (mut self) -> Fraction {
        let mut i = 0;
        let n = PRIMES.len();
        while i < n {
            if self.denominator < PRIMES[i] {break;}
            if self.numerator % PRIMES[i] == 0 && self.denominator % PRIMES[i] == 0 {
                self.numerator /= PRIMES[i];
                self.denominator /= PRIMES[i];
                i = 0;
            } else {
                i += 1;
            }
        }
        self
    }
    
    fn sum (num1: i32, den1: i32, num2: i32, den2: i32) -> Fraction {
        Fraction::new(num1*den2 + num2*den1, den1 * den2).simplify()
    }
    fn sub (num1: i32, den1: i32, num2: i32, den2: i32) -> Fraction {
        Fraction::new(num1*den2 - num2*den1, den1 * den2).simplify()
    }
    fn mul (num1: i32, den1: i32, num2: i32, den2: i32) -> Fraction {
        Fraction::new(num1*num2, den1*den2).simplify()
    }
    fn div (num1: i32, den1: i32, num2: i32, den2: i32) -> Fraction {
        Fraction::mul(num1, den1, den2, num2)
    }
}

impl Add for Fraction {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Fraction::sum(self.numerator, self.denominator, other.numerator, other.denominator)
    }
}
impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = Fraction::sum(self.numerator, self.denominator, other.numerator, other.denominator);
    }
}
impl Sub for Fraction {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Fraction::sub(self.numerator, self.denominator, other.numerator, other.denominator)
    }
}
impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = Fraction::sub(self.numerator, self.denominator, other.numerator, other.denominator);
    }
}
impl Mul for Fraction {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Fraction::mul(self.numerator, self.denominator, other.numerator, other.denominator)
    }
}
impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = Fraction::mul(self.numerator, self.denominator, other.numerator, other.denominator);
    }
}
impl Div for Fraction {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Fraction::div(self.numerator, self.denominator, other.numerator, other.denominator)
    }
}
impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = Fraction::div(self.numerator, self.denominator, other.numerator, other.denominator);
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let a = self.numerator*other.denominator;
        let b = other.numerator*self.denominator;
        a == b
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.numerator*other.denominator;
        let b = other.numerator*self.denominator;
        a.cmp(&b)
    }
}
impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{})", self.numerator, self.denominator)
    }
}