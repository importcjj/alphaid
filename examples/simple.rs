use alphaid::AlphaId;

fn main() {
    let a = AlphaId::<u64>::new();
    let v = a.encode(730087).unwrap();
    println!("{}", unsafe { std::str::from_utf8_unchecked(&v) });

    println!("{:?}", a.decode(v));
}
