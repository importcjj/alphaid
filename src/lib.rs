const MAXLEN: usize = 6;
const BASE: usize = 62;
static ALPHABET: &'static [u8;BASE]= b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";


#[derive(Debug)]
pub struct EncodeError;


#[derive(Debug)]
pub struct DecodeError;

pub fn encode(mut n: i64) -> Result<Vec<u8>, EncodeError> {
    let pad = MAXLEN - 1;
    let mut out = vec![];
    if pad > 0 {
        n += BASE.pow(pad as u32) as i64;
    }
    
    let mut t = (n as f64).log(BASE as f64) as i64;
    for _ in (0..=t).rev() {
        let bcp = BASE.pow(t as u32) as i64;
        let a = (n / bcp) % BASE as i64;
        out.push(ALPHABET[a as usize]);
        n -= a * bcp;
        t -= 1;
    }
    Ok(out)
}

pub fn decode(s: &[u8]) -> Result<i64, DecodeError> {    
    let length= s.len();
    if length == 0 {
        return Err(DecodeError)
    }

    let mut r = 0;
    let l = length - 1;


    for i in (0..=l).rev() {
        let b = l - i;
        let bcp = BASE.pow(b as u32) as i64;
        match ALPHABET.iter().position(|&x| x == s[i]) {
            Some(i) => r += bcp * i as i64,
            None => return Err(DecodeError)
        }
    }

    let pad = MAXLEN - 1;
    if pad > 0 {
        r -= BASE.pow(pad as u32) as i64;
    }
    Ok(r)
}
