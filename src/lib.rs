//! Generate Youtube-Like IDs with Rust.
//!
//! # Example
//!
//! ```rust
//! use alphaid::AlphaId;
//!
//! let alphaid = AlphaId::<u32>::new();
//! assert_eq!(alphaid.encode(0), Ok(b"a".to_vec()));
//! assert_eq!(alphaid.encode(1), Ok(b"b".to_vec()));
//! assert_eq!(alphaid.encode(1350997667), Ok(b"90F7qb".to_vec()));
//!
//! assert_eq!(alphaid.decode(b"a"), Ok(0));
//! assert_eq!(alphaid.decode(b"b"), Ok(1));
//! assert_eq!(alphaid.decode(b"90F7qb"), Ok(1350997667));
//!
//! let alphaid = AlphaId::<u32>::builder().pad(2).build();
//! assert_eq!(alphaid.encode(0), Ok(b"ab".to_vec()));
//! assert_eq!(alphaid.decode(b"ab"), Ok(0));
//!
//! let alphaid = AlphaId::<u32>::builder().pad(2)
//!     .chars("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().to_vec())
//!     .build();
//! assert_eq!(alphaid.encode(0), Ok(b"AB".to_vec()));
//! assert_eq!(alphaid.decode(b"AB"), Ok(0));
//!```
use num::{Bounded, FromPrimitive, Integer, NumCast, ToPrimitive};
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait UnsignedInteger:
    Integer + Bounded + ToPrimitive + Clone + FromPrimitive + NumCast + Copy
{
}

impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for usize {}
impl UnsignedInteger for u128 {}

static DEFAULT_SEED: &'static str =
    "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

#[derive(Debug, PartialEq)]
pub enum AlphaIdError {
    InvalidNumber,
    Overflow,
    UnexpectedChar,
}

/// A builder for a `AlphaId`.
pub struct Builder<T: UnsignedInteger = u128> {
    chars: Option<Vec<u8>>,
    pad: Option<u32>,
    _data: PhantomData<T>,
}

impl<T: UnsignedInteger> Default for Builder<T> {
    fn default() -> Self {
        Self {
            chars: None,
            pad: None,
            _data: PhantomData,
        }
    }
}

impl<T: UnsignedInteger> Builder<T> {
    /// Constructs a new `Builder`.
    ///
    /// Parameters are initialized with their default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the characters set.
    ///
    /// Default to `abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-_`.
    ///
    /// # Panics
    ///
    /// Panics if chars' size is less than `16`.
    pub fn chars(mut self, chars: Vec<u8>) -> Self {
        assert!(chars.len() > 16, "chars size must large than 16");
        self.chars = Some(chars);
        self
    }

    /// Sets the pad which specifies the minimum
    /// length of the encoded result.Result
    ///
    /// Default to 1.
    ///
    /// # Panics
    ///
    /// Panics if pad is less than 1.
    pub fn pad(mut self, pad: u32) -> Self {
        assert!(pad > 0, "pad must large than 1");
        self.pad = Some(pad);
        self
    }

    /// Consumes the builder, returning a `AlphaId`.
    ///
    /// # Panics
    ///
    /// Panics if there are duplicate characters in chars.
    pub fn build(self) -> AlphaId<T> {
        let chars = self
            .chars
            .unwrap_or_else(|| DEFAULT_SEED.as_bytes().to_vec());

        let index: HashMap<u8, T> = chars
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, T::from_usize(i).unwrap()))
            .collect();

        assert!(
            chars.len() == index.len(),
            "duplicate characters are not allowed"
        );
        let base = T::from_usize(chars.len()).expect("primitive number type");
        let a: f64 = <f64 as NumCast>::from(T::max_value()).unwrap();
        let b: f64 = <f64 as NumCast>::from(base.clone()).unwrap();
        let max_pow_i = a.log(b) as u32;
        AlphaId {
            chars,
            index,
            base,
            pad: self.pad.unwrap_or(1),
            max_pow_i,
        }
    }
}

/// Used for encoding and decoding.
pub struct AlphaId<T: UnsignedInteger = u128> {
    chars: Vec<u8>,
    index: HashMap<u8, T>,
    base: T,
    pad: u32,
    max_pow_i: u32,
}

impl<T: UnsignedInteger> AlphaId<T> {
    /// Returns a builder type to configure a new `AlphaId`.
    pub fn builder() -> Builder<T> {
        Builder::new()
    }

    /// Creates a new `AlphaId` with a default configuration.
    pub fn new() -> Self {
        Builder::new().build()
    }

    /// Encode the numbers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use alphaid::AlphaId;
    ///
    /// let alphaid = AlphaId::<u32>::new();
    /// assert_eq!(alphaid.encode(0), Ok(b"a".to_vec()));
    /// assert_eq!(alphaid.encode(1), Ok(b"b".to_vec()));
    /// assert_eq!(alphaid.encode(1350997667), Ok(b"90F7qb".to_vec()));
    /// ```
    pub fn encode(&self, mut n: T) -> Result<Vec<u8>, AlphaIdError> {
        let mut out = vec![];
        let mut i = 0;
        loop {
            i += 1;
            if self.pad > 1 && self.pad == i {
                n = n + T::one();
            }

            if n.is_zero() {
                if i <= self.pad {
                    out.push(self.chars[0]);
                    continue;
                }
                break;
            }

            let a = n % self.base;
            out.push(self.chars[a.to_usize().ok_or_else(|| AlphaIdError::InvalidNumber)?]);
            n = n / self.base;
        }

        Ok(out)
    }

    /// Decode into numbers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use alphaid::AlphaId;
    ///
    /// let alphaid = AlphaId::<u32>::new();
    /// assert_eq!(alphaid.decode(b"a"), Ok(0));
    /// assert_eq!(alphaid.decode(b"b"), Ok(1));
    /// assert_eq!(alphaid.decode(b"90F7qb"), Ok(1350997667));
    ///```
    pub fn decode<V: AsRef<[u8]>>(&self, v: V) -> Result<T, AlphaIdError> {
        let v = v.as_ref();
        let mut i: usize = 0;
        let mut n = T::zero();
        let mut unpad = self.pad > 1;
        let mut prev = T::zero();
        let num63 = T::from_u32(63).unwrap();

        while i < v.len() {
            match self.index.get(&v[i as usize]) {
                Some(t) => {
                    let mut x = *t;

                    if unpad && i + 1 >= self.pad as usize {
                        if i > 1 {
                            n = n + num::pow(self.base, i - 1) * (num63 - prev);
                        }

                        if !x.is_zero() {
                            unpad = false;
                            x = x - T::one();
                        }
                    };

                    prev = *t;

                    if x.is_zero() {
                        i += 1;
                        continue;
                    }

                    if i > self.max_pow_i as usize {
                        return Err(AlphaIdError::Overflow);
                    }

                    let pow = num::pow(self.base, i);
                    if T::max_value().div(pow) < x {
                        return Err(AlphaIdError::Overflow);
                    }
                    let add = pow * x;
                    if T::max_value() - n < add {
                        return Err(AlphaIdError::Overflow);
                    }
                    n = n + add;
                }
                None => return Err(AlphaIdError::UnexpectedChar),
            }
            i += 1;
        }

        Ok(n)
    }
}
