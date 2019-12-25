use std::collections::HashMap;

static DEFAULT_SEED: &'static str =
    "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Overflow,
    UnexpectedChar,
}

pub struct Builder {
    chars: Option<Vec<u8>>,
    pad: Option<u32>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            chars: None,
            pad: None,
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chars(mut self, chars: Vec<u8>) -> Self {
        assert!(chars.len() > 16, "chars size must large than 16");
        self.chars = Some(chars);
        self
    }

    pub fn pad(mut self, pad: u32) -> Self {
        assert!(pad > 0, "pad must large than 1");
        self.pad = Some(pad);
        self
    }

    pub fn build(self) -> AlphaId {
        let chars = self
            .chars
            .unwrap_or_else(|| DEFAULT_SEED.as_bytes().to_vec());
        let index: HashMap<u8, u128> = chars
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i as u128))
            .collect();

        assert!(
            chars.len() == index.len(),
            "duplicate characters are not allowed"
        );
        let base = chars.len() as u128;
        let max_pow_i = (u128::max_value() as f64).log(base as f64) as u32;
        AlphaId {
            chars,
            index,
            base,
            pad: self.pad.unwrap_or(1),
            max_pow_i,
        }
    }
}

pub struct AlphaId {
    chars: Vec<u8>,
    index: HashMap<u8, u128>,
    base: u128,
    pad: u32,
    max_pow_i: u32,
}

impl AlphaId {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn new() -> Self {
        Builder::new().build()
    }

    pub fn encode(&self, mut n: u128) -> Vec<u8> {
        let mut out = vec![];
        let mut i = 0;
        loop {
            i += 1;
            if self.pad > 1 && self.pad == i {
                n += 1;
            }

            if n == 0 {
                if i <= self.pad {
                    out.push(self.chars[0]);
                    continue;
                }
                break;
            }

            let a = n % self.base;
            out.push(self.chars[a as usize]);
            n = n / self.base;
        }

        out
    }

    pub fn decode<V: AsRef<[u8]>>(&self, v: V) -> Result<u128, DecodeError> {
        let v = v.as_ref();
        let mut i = 0;
        let mut n = 0;
        let mut unpad = self.pad > 1;
        let mut prev = 0;

        while i < v.len() as u32 {
            match self.index.get(&v[i as usize]) {
                Some(t) => {
                    let mut x = *t as u128;

                    if unpad && i >= self.pad - 1 {
                        if i > 1 {
                            n += self.base.pow(i - 1) * (63 - prev);
                        }

                        match x {
                            0 => (),
                            _ => {
                                unpad = false;
                                x -= 1;
                            }
                        }
                    };

                    prev = *t;

                    if x == 0 {
                        i += 1;
                        continue;
                    }

                    if i > self.max_pow_i {
                        return Err(DecodeError::Overflow);
                    }

                    let pow = self.base.pow(i);
                    if u128::max_value() / pow < x {
                        return Err(DecodeError::Overflow);
                    }
                    let add = pow * x;
                    if u128::max_value() - n < add {
                        return Err(DecodeError::Overflow);
                    }
                    n += add;
                }
                None => return Err(DecodeError::UnexpectedChar),
            }
            i += 1;
        }

        Ok(n)
    }
}
