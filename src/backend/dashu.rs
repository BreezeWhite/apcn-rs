use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use dashu::base::Abs;
use dashu::float::Context;
use dashu::float::FBig;
use dashu::float::round::mode::HalfEven;
use dashu::integer::IBig;

// We use base 2 for maximum calculation speed, matching the Rug/GMP backend.
type DFloat = FBig<HalfEven, 2>;

fn bits_to_digits(bits: u32) -> usize {
    (bits as f64 / std::f64::consts::LOG2_10).ceil() as usize
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigInt {
    pub(crate) val: IBig,
}

impl BigInt {
    pub fn abs(&self) -> Self {
        BigInt {
            val: self.val.clone().abs(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        BigInt {
            val: self.val.pow(exp as usize),
        }
    }

    pub fn assign<T>(&mut self, value: T)
    where
        Self: From<T>,
    {
        *self = Self::from(value);
    }

    pub fn mul_from(&mut self, other: &Self) {
        self.val *= &other.val;
    }
}

// Conversions for BigInt
impl From<u64> for BigInt {
    fn from(val: u64) -> Self {
        BigInt {
            val: IBig::from(val),
        }
    }
}

impl From<i32> for BigInt {
    fn from(val: i32) -> Self {
        BigInt {
            val: IBig::from(val),
        }
    }
}

impl From<i64> for BigInt {
    fn from(val: i64) -> Self {
        BigInt {
            val: IBig::from(val),
        }
    }
}

impl From<usize> for BigInt {
    fn from(val: usize) -> Self {
        BigInt {
            val: IBig::from(val),
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
        write!(f, "{}", self.val)
    }
}

// Operators for BigInt
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        self.val += &rhs.val;
    }
}

impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        self.val += rhs.val;
    }
}

impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: &BigInt) {
        self.val -= &rhs.val;
    }
}

impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: &BigInt) {
        self.val *= &rhs.val;
    }
}

impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        self.val *= rhs.val;
    }
}

impl MulAssign<u64> for BigInt {
    fn mul_assign(&mut self, rhs: u64) {
        self.val *= IBig::from(rhs);
    }
}

impl MulAssign<i32> for BigInt {
    fn mul_assign(&mut self, rhs: i32) {
        self.val *= IBig::from(rhs);
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        BigInt {
            val: self.val + rhs.val,
        }
    }
}

impl Sub<BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: BigInt) -> BigInt {
        BigInt {
            val: self.val - rhs.val,
        }
    }
}

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            val: self.val * rhs.val,
        }
    }
}

impl<'a, 'b> Mul<&'b BigInt> for &'a BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &'b BigInt) -> BigInt {
        BigInt {
            val: &self.val * &rhs.val,
        }
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> BigInt {
        BigInt {
            val: self.val * IBig::from(rhs),
        }
    }
}

impl<'a> Mul<u64> for &'a BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> BigInt {
        BigInt {
            val: &self.val * IBig::from(rhs),
        }
    }
}

// -------------------------------------------------------------
// BigFloat Wrapper
// -------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct BigFloat {
    pub(crate) val: DFloat,
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

    pub fn precision(&self) -> u32 {
        self.val.precision() as u32
    }

    pub fn set_prec(&mut self, prec: u32) {
        self.val = self.val.clone().with_precision(prec as usize).value();
    }

    pub fn assign<T>(&mut self, value: T)
    where
        Self: From<T>,
    {
        *self = Self::from(value);
    }

    pub fn pow_assign(&mut self, exp: u32) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let mut res = self.clone();
        for _ in 1..exp {
            res.val = ctxt.mul(res.val.repr(), self.val.repr()).value();
        }
        self.val = res.val;
    }

    pub fn sqrt_mut(&mut self) {
        use dashu::base::SquareRoot;
        self.val = self.val.sqrt();
    }

    pub fn exp(&self) -> Self {
        BigFloat {
            val: self.val.exp(),
        }
    }

    pub fn to_int(&self) -> BigInt {
        BigInt {
            val: self.val.clone().to_int().value(),
        }
    }


    pub fn sqrt(self) -> Self {
        use dashu::base::SquareRoot;
        BigFloat {
            val: self.val.sqrt(),
        }
    }

    pub fn ln(self) -> Self {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.ln(&self.val.repr()).value(),
        }
    }

    pub fn to_fixed_string(&self) -> String {
        self.to_string()
    }
}

// FromPrec implementations for BigFloat
impl FromPrec<&BigFloat> for BigFloat {
    fn from_prec(prec: u32, val: &BigFloat) -> Self {
        BigFloat {
            val: val.val.clone().with_precision(prec as usize).value(),
        }
    }
}

impl FromPrec<BigFloat> for BigFloat {
    fn from_prec(prec: u32, val: BigFloat) -> Self {
        BigFloat {
            val: val.val.with_precision(prec as usize).value(),
        }
    }
}

impl FromPrec<&BigInt> for BigFloat {
    fn from_prec(prec: u32, val: &BigInt) -> Self {
        let ctxt = Context::<HalfEven>::new(prec as usize);
        BigFloat {
            val: ctxt.convert_int::<2>(val.val.clone()).value(),
        }
    }
}

impl FromPrec<BigInt> for BigFloat {
    fn from_prec(prec: u32, val: BigInt) -> Self {
        let ctxt = Context::<HalfEven>::new(prec as usize);
        BigFloat {
            val: ctxt.convert_int::<2>(val.val).value(),
        }
    }
}

impl FromPrec<f64> for BigFloat {
    fn from_prec(prec: u32, val: f64) -> Self {
        let raw = DFloat::try_from(val).unwrap();
        BigFloat {
            val: raw.with_precision(prec as usize).value(),
        }
    }
}

impl FromPrec<u32> for BigFloat {
    fn from_prec(prec: u32, val: u32) -> Self {
        let ctxt = Context::<HalfEven>::new(prec as usize);
        BigFloat {
            val: ctxt.convert_int::<2>(IBig::from(val)).value(),
        }
    }
}

impl FromPrec<i32> for BigFloat {
    fn from_prec(prec: u32, val: i32) -> Self {
        let ctxt = Context::<HalfEven>::new(prec as usize);
        BigFloat {
            val: ctxt.convert_int::<2>(IBig::from(val)).value(),
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
            val: DFloat::try_from(val).unwrap(),
        }
    }
}

// Display - Custom fast binary-to-decimal radix formatting
impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.val.repr().is_zero() {
            return write!(f, "0.0");
        }

        let prec = self.precision();
        let digits = bits_to_digits(prec);

        let sign = self.val.sign();
        let (significand, exponent) = self.val.clone().into_repr().into_parts();
        let abs_sig = significand.abs();

        // Scale by 10^digits
        let ten_pow = IBig::from(10).pow(digits);
        let scaled = if exponent >= 0 {
            (abs_sig * ten_pow) << (exponent as usize)
        } else {
            (abs_sig * ten_pow) >> ((-exponent) as usize)
        };

        let mut s = scaled.to_string();
        let mut prefix = String::new();
        if sign == dashu::base::Sign::Negative {
            prefix.push('-');
        }

        let l = s.len();
        if l > digits {
            let dot_idx = l - digits;
            s.insert(dot_idx, '.');
            prefix.push_str(&s);
        } else {
            prefix.push_str("0.");
            let zeros = digits - l;
            for _ in 0..zeros {
                prefix.push('0');
            }
            prefix.push_str(&s);
        }
        write!(f, "{}", prefix)
    }
}

// Operators for BigFloat
impl AddAssign<&BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: &BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.add(self.val.repr(), rhs.val.repr()).value();
    }
}

impl AddAssign<BigFloat> for BigFloat {
    fn add_assign(&mut self, rhs: BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.add(self.val.repr(), rhs.val.repr()).value();
    }
}

impl SubAssign<&BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: &BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.sub(self.val.repr(), rhs.val.repr()).value();
    }
}

impl SubAssign<BigFloat> for BigFloat {
    fn sub_assign(&mut self, rhs: BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.sub(self.val.repr(), rhs.val.repr()).value();
    }
}

impl MulAssign<&BigFloat> for BigFloat {
    fn mul_assign(&mut self, rhs: &BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.mul(self.val.repr(), rhs.val.repr()).value();
    }
}

impl MulAssign<BigFloat> for BigFloat {
    fn mul_assign(&mut self, rhs: BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.mul(self.val.repr(), rhs.val.repr()).value();
    }
}

impl MulAssign<u32> for BigFloat {
    fn mul_assign(&mut self, rhs: u32) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        self.val = ctxt.mul(self.val.repr(), rhs_float.repr()).value();
    }
}

impl MulAssign<i32> for BigFloat {
    fn mul_assign(&mut self, rhs: i32) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        self.val = ctxt.mul(self.val.repr(), rhs_float.repr()).value();
    }
}

impl DivAssign<&BigFloat> for BigFloat {
    fn div_assign(&mut self, rhs: &BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.div(self.val.repr(), rhs.val.repr()).value();
    }
}

impl DivAssign<BigFloat> for BigFloat {
    fn div_assign(&mut self, rhs: BigFloat) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        self.val = ctxt.div(self.val.repr(), rhs.val.repr()).value();
    }
}

impl DivAssign<u32> for BigFloat {
    fn div_assign(&mut self, rhs: u32) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        self.val = ctxt.div(self.val.repr(), rhs_float.repr()).value();
    }
}

impl DivAssign<i32> for BigFloat {
    fn div_assign(&mut self, rhs: i32) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        self.val = ctxt.div(self.val.repr(), rhs_float.repr()).value();
    }
}

impl DivAssign<&BigInt> for BigFloat {
    fn div_assign(&mut self, rhs: &BigInt) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val.clone()).value();
        self.val = ctxt.div(self.val.repr(), rhs_float.repr()).value();
    }
}

impl DivAssign<BigInt> for BigFloat {
    fn div_assign(&mut self, rhs: BigInt) {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val).value();
        self.val = ctxt.div(self.val.repr(), rhs_float.repr()).value();
    }
}

impl Add<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.add(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl<'a, 'b> Add<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: &'b BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.add(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl Sub<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.sub(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl<'a, 'b> Sub<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: &'b BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.sub(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl Mul<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl<'a, 'b> Mul<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: &'b BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl Mul<u32> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: u32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a> Mul<u32> for &'a BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: u32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl Mul<i32> for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: i32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a> Mul<i32> for &'a BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: i32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.mul(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl Div<BigFloat> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl<'a, 'b> Div<&'b BigFloat> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'b BigFloat) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs.val.repr()).value(),
        }
    }
}

impl Div<i32> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: i32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a> Div<i32> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: i32) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(IBig::from(rhs)).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

// Mul of u32 and BigFloat
impl Mul<BigFloat> for u32 {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        rhs * self
    }
}

impl<'a> Mul<&'a BigFloat> for u32 {
    type Output = BigFloat;
    fn mul(self, rhs: &'a BigFloat) -> BigFloat {
        rhs * self
    }
}

// Mul of i32 and BigFloat
impl Mul<BigFloat> for i32 {
    type Output = BigFloat;
    fn mul(self, rhs: BigFloat) -> BigFloat {
        rhs * self
    }
}

impl<'a> Mul<&'a BigFloat> for i32 {
    type Output = BigFloat;
    fn mul(self, rhs: &'a BigFloat) -> BigFloat {
        rhs * self
    }
}

// Mul of u32/i32 and BigInt
impl Mul<BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            val: IBig::from(self) * rhs.val,
        }
    }
}

impl<'a> Mul<&'a BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> BigInt {
        BigInt {
            val: IBig::from(self) * &rhs.val,
        }
    }
}

impl Mul<BigInt> for i32 {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> BigInt {
        BigInt {
            val: IBig::from(self) * rhs.val,
        }
    }
}

impl<'a> Mul<&'a BigInt> for i32 {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> BigInt {
        BigInt {
            val: IBig::from(self) * &rhs.val,
        }
    }
}

// Div of BigFloat by BigInt
impl Div<BigInt> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigInt) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a> Div<&'a BigInt> for BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'a BigInt) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val.clone()).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a> Div<BigInt> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: BigInt) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}

impl<'a, 'b> Div<&'b BigInt> for &'a BigFloat {
    type Output = BigFloat;
    fn div(self, rhs: &'b BigInt) -> BigFloat {
        let ctxt = Context::<HalfEven>::new(self.precision() as usize);
        let rhs_float = ctxt.convert_int::<2>(rhs.val.clone()).value();
        BigFloat {
            val: ctxt.div(self.val.repr(), rhs_float.repr()).value(),
        }
    }
}
