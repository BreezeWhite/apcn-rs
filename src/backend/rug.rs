use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rug::float::Round;
use rug::{Float, Integer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigInt {
    pub(crate) num: Integer,
}

impl BigInt {
    pub fn abs(&self) -> Self {
        BigInt {
            num: self.num.clone().abs(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        use rug::ops::Pow;
        BigInt {
            num: <Integer as Pow<u32>>::pow(self.num.clone(), exp),
        }
    }

    pub fn assign<T>(&mut self, value: T)
    where
        Self: From<T>,
    {
        *self = Self::from(value);
    }

    pub fn mul_from(&mut self, other: &Self) {
        use rug::ops::MulFrom;
        self.num.mul_from(&other.num);
    }
}

// Conversions for BigInt
impl From<u64> for BigInt {
    fn from(val: u64) -> Self {
        BigInt {
            num: Integer::from(val),
        }
    }
}

impl From<i32> for BigInt {
    fn from(val: i32) -> Self {
        BigInt {
            num: Integer::from(val),
        }
    }
}

impl From<i64> for BigInt {
    fn from(val: i64) -> Self {
        BigInt {
            num: Integer::from(val),
        }
    }
}

impl From<usize> for BigInt {
    fn from(val: usize) -> Self {
        BigInt {
            num: Integer::from(val),
        }
    }
}

impl From<&BigInt> for BigInt {
    fn from(val: &BigInt) -> Self {
        val.clone()
    }
}

// Display
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

// Operators for BigInt
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        self.num += &rhs.num;
    }
}

impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        self.num += rhs.num;
    }
}

impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: &BigInt) {
        self.num -= &rhs.num;
    }
}

impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: &BigInt) {
        self.num *= &rhs.num;
    }
}

impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        self.num *= rhs.num;
    }
}

impl MulAssign<u64> for BigInt {
    fn mul_assign(&mut self, rhs: u64) {
        self.num *= rhs;
    }
}

impl MulAssign<i32> for BigInt {
    fn mul_assign(&mut self, rhs: i32) {
        self.num *= rhs;
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        BigInt {
            num: self.num + rhs.num,
        }
    }
}

impl Sub<BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: BigInt) -> BigInt {
        BigInt {
            num: self.num - rhs.num,
        }
    }
}

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            num: self.num * rhs.num,
        }
    }
}

impl<'a, 'b> Mul<&'b BigInt> for &'a BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &'b BigInt) -> BigInt {
        BigInt {
            num: Integer::from(&self.num * &rhs.num),
        }
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> BigInt {
        BigInt {
            num: self.num * rhs,
        }
    }
}

impl<'a> Mul<u64> for &'a BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> BigInt {
        BigInt {
            num: Integer::from(&self.num * rhs),
        }
    }
}

// -------------------------------------------------------------
// BigFloat Wrapper
// -------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct BigFloat {
    pub(crate) num: Float,
}

pub trait FromPrec<T> {
    fn from_prec(prec: u32, val: T) -> Self;
}

impl BigFloat {
    pub fn with_val<T>(prec: u32, val: T) -> Self
    where
        Self: FromPrec<T>,
    {
        FromPrec::from_prec(prec, val)
    }

    pub fn to_int(&self) -> BigInt {
        let (int, _) = self.num.clone().to_integer_round(Round::Nearest).unwrap();
        BigInt { num: int }
    }


    pub fn to_fixed_string(&self) -> String {
        let s = self.num.to_string_radix_round(10, None, Round::Nearest);
        if !s.contains('e') && !s.contains('E') {
            return s;
        }

        let parts: Vec<&str> = s.split(|c| c == 'e' || c == 'E').collect();
        let mantissa = parts[0];
        let exp: i64 = parts[1].parse().unwrap();

        let is_neg = mantissa.starts_with('-');
        let abs_mantissa = if is_neg { &mantissa[1..] } else { mantissa };

        let dot_pos = abs_mantissa.find('.');
        let digits = abs_mantissa.replace('.', "");

        let mut exp_adj = exp;
        if let Some(pos) = dot_pos {
            exp_adj -= (abs_mantissa.len() - 1 - pos) as i64;
        }

        let mut result = String::new();
        if is_neg {
            result.push('-');
        }

        if exp_adj >= 0 {
            result.push_str(&digits);
            for _ in 0..exp_adj {
                result.push('0');
            }
        } else {
            let shift = (-exp_adj) as usize;
            if shift < digits.len() {
                let split_pos = digits.len() - shift;
                result.push_str(&digits[..split_pos]);
                result.push('.');
                result.push_str(&digits[split_pos..]);
            } else {
                result.push_str("0.");
                let zeros = shift - digits.len();
                for _ in 0..zeros {
                    result.push('0');
                }
                result.push_str(&digits);
            }
        }
        result
    }

    pub fn precision(&self) -> u32 {
        self.num.prec()
    }

    pub fn set_prec(&mut self, prec: u32) {
        self.num.set_prec(prec);
    }

    pub fn assign<T>(&mut self, value: T)
    where
        Self: From<T>,
    {
        *self = Self::from(value);
    }

    pub fn pow_assign(&mut self, exp: u32) {
        use rug::ops::PowAssign;
        self.num.pow_assign(exp);
    }

    pub fn sqrt_mut(&mut self) {
        self.num.sqrt_mut();
    }

    pub fn sqrt(self) -> Self {
        BigFloat {
            num: self.num.sqrt(),
        }
    }

    pub fn exp(self) -> Self {
        BigFloat {
            num: self.num.exp(),
        }
    }

    pub fn ln(self) -> Self {
        BigFloat { num: self.num.ln() }
    }
}

// FromPrec implementations for BigFloat
impl FromPrec<&BigFloat> for BigFloat {
    fn from_prec(prec: u32, val: &BigFloat) -> Self {
        BigFloat {
            num: Float::with_val(prec, &val.num),
        }
    }
}

impl FromPrec<BigFloat> for BigFloat {
    fn from_prec(prec: u32, val: BigFloat) -> Self {
        BigFloat {
            num: Float::with_val(prec, val.num),
        }
    }
}

impl FromPrec<&BigInt> for BigFloat {
    fn from_prec(prec: u32, val: &BigInt) -> Self {
        BigFloat {
            num: Float::with_val(prec, &val.num),
        }
    }
}

impl FromPrec<BigInt> for BigFloat {
    fn from_prec(prec: u32, val: BigInt) -> Self {
        BigFloat {
            num: Float::with_val(prec, val.num),
        }
    }
}

impl FromPrec<f64> for BigFloat {
    fn from_prec(prec: u32, val: f64) -> Self {
        BigFloat {
            num: Float::with_val(prec, val),
        }
    }
}

impl FromPrec<u32> for BigFloat {
    fn from_prec(prec: u32, val: u32) -> Self {
        BigFloat {
            num: Float::with_val(prec, val),
        }
    }
}

impl FromPrec<i32> for BigFloat {
    fn from_prec(prec: u32, val: i32) -> Self {
        BigFloat {
            num: Float::with_val(prec, val),
        }
    }
}

// From conversions for BigFloat (needed for assign method)
impl From<&BigFloat> for BigFloat {
    fn from(val: &BigFloat) -> Self {
        val.clone()
    }
}

impl From<f64> for BigFloat {
    fn from(val: f64) -> Self {
        BigFloat {
            num: Float::with_val(53, val),
        }
    }
}

// Display
impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

// Operators for BigFloat
impl AddAssign<&BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: &BigFloat) {
        self.num += &rhs.num;
    }
}

impl AddAssign<BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: BigFloat) {
        self.num += rhs.num;
    }
}

impl SubAssign<&BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: &BigFloat) {
        self.num -= &rhs.num;
    }
}

impl SubAssign<BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: BigFloat) {
        self.num -= rhs.num;
    }
}

impl MulAssign<&BigFloat> for BigFloat {
    fn mul_assign(&mut self, rhs: &BigFloat) {
        self.num *= &rhs.num;
    }
}

impl MulAssign<BigFloat> for BigFloat {
    fn mul_assign(&mut self, rhs: BigFloat) {
        self.num *= rhs.num;
    }
}

impl MulAssign<u32> for BigFloat {
    fn mul_assign(&mut self, rhs: u32) {
        self.num *= rhs;
    }
}

impl MulAssign<i32> for BigFloat {
    fn mul_assign(&mut self, rhs: i32) {
        self.num *= rhs;
    }
}

impl DivAssign<&BigFloat> for BigFloat {
    fn div_assign(&mut self, rhs: &BigFloat) {
        self.num /= &rhs.num;
    }
}

impl DivAssign<BigFloat> for BigFloat {
    fn div_assign(&mut self, rhs: BigFloat) {
        self.num /= rhs.num;
    }
}

impl DivAssign<u32> for BigFloat {
    fn div_assign(&mut self, rhs: u32) {
        self.num /= rhs;
    }
}

impl DivAssign<i32> for BigFloat {
    fn div_assign(&mut self, rhs: i32) {
        self.num /= rhs;
    }
}

impl DivAssign<&BigInt> for BigFloat {
    fn div_assign(&mut self, rhs: &BigInt) {
        self.num /= &rhs.num;
    }
}

impl DivAssign<BigInt> for BigFloat {
    fn div_assign(&mut self, rhs: BigInt) {
        self.num /= rhs.num;
    }
}

impl Add<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self.num + rhs.num,
        }
    }
}

impl<'a, 'b> Add<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: &'b BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num + &rhs.num),
        }
    }
}

impl Sub<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self.num - rhs.num,
        }
    }
}

impl<'a, 'b> Sub<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: &'b BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num - &rhs.num),
        }
    }
}

impl Mul<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self.num * rhs.num,
        }
    }
}

impl<'a, 'b> Mul<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: &'b BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num * &rhs.num),
        }
    }
}

impl Mul<u32> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: u32) -> BigFloat {
        BigFloat {
            num: self.num * rhs,
        }
    }
}

impl<'a> Mul<u32> for &'a BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: u32) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num * rhs),
        }
    }
}

impl Div<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self.num / rhs.num,
        }
    }
}

impl<'a, 'b> Div<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'b BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num / &rhs.num),
        }
    }
}

impl Div<i32> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: i32) -> BigFloat {
        BigFloat {
            num: self.num / rhs,
        }
    }
}

impl<'a> Div<i32> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: i32) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num / rhs),
        }
    }
}

// Mul of u32 and BigFloat
impl Mul<BigFloat> for u32 {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self * rhs.num,
        }
    }
}

impl<'a> Mul<&'a BigFloat> for u32 {
    type Output = BigFloat;
    fn mul(self, rhs: &'a BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(rhs.precision(), self * &rhs.num),
        }
    }
}

// Mul of i32 and BigFloat
impl Mul<BigFloat> for i32 {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        BigFloat {
            num: self * rhs.num,
        }
    }
}

impl<'a> Mul<&'a BigFloat> for i32 {
    type Output = BigFloat;
    fn mul(self, rhs: &'a BigFloat) -> BigFloat {
        BigFloat {
            num: Float::with_val(rhs.precision(), self * &rhs.num),
        }
    }
}

// Mul of u32/i32 and BigInt
impl Mul<BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            num: Integer::from(self * &rhs.num),
        }
    }
}

impl<'a> Mul<&'a BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> BigInt {
        BigInt {
            num: Integer::from(self * &rhs.num),
        }
    }
}

impl Mul<BigInt> for i32 {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            num: Integer::from(self * &rhs.num),
        }
    }
}

impl<'a> Mul<&'a BigInt> for i32 {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> BigInt {
        BigInt {
            num: Integer::from(self * &rhs.num),
        }
    }
}

// Div of BigFloat by BigInt
impl Div<BigInt> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigInt) -> BigFloat {
        BigFloat {
            num: self.num / rhs.num,
        }
    }
}

impl<'a> Div<&'a BigInt> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'a BigInt) -> BigFloat {
        BigFloat {
            num: self.num / &rhs.num,
        }
    }
}

impl<'a> Div<BigInt> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigInt) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num / rhs.num),
        }
    }
}

impl<'a, 'b> Div<&'b BigInt> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'b BigInt) -> BigFloat {
        BigFloat {
            num: Float::with_val(self.precision(), &self.num / &rhs.num),
        }
    }
}
