const MINLEN: usize = 6;
const BASE: usize = 62;
static ALPHABET: &'static [u8;BASE]= b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";



fn main() {
    let x = encode(1).unwrap();
    println!("{:?}", std::str::from_utf8(&x));
    
    let i = decode(b"baaaab").unwrap();
    println!("{}", i);
}

#[derive(Debug)]
pub struct EncodeError;


#[derive(Debug)]
pub struct DecodeError;

pub fn encode(mut n: i64) -> Result<Vec<u8>, EncodeError> {
    let pad = MINLEN - 1;
    let mut out = vec![];
    if pad > 0 {
        n += BASE.pow(pad as u32) as i64;
    }
    
    let length = (n as f64).log(BASE as f64) as i64;
    for i in (0..=length).rev() {
        let bcp = BASE.pow(i as u32) as i64;
        let a = n / bcp;
        out.push(ALPHABET[a as usize]);
        n -= a * bcp;
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
            Some(t) => r += bcp * t as i64,
            None => return Err(DecodeError)
        }
    }

    let pad = MINLEN - 1;
    if pad > 0 {
        r -= BASE.pow(pad as u32) as i64;
    }
    Ok(r)
}
