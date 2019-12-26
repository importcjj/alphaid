use alphaid::{AlphaId, DecodeError};
#[test]
fn test_encode_basic() {
    let alphaid = AlphaId::new();
    assert_eq!(alphaid.encode(0), b"a");
    assert_eq!(alphaid.encode(1), b"b");
    assert_eq!(alphaid.encode(62), b"-");
    assert_eq!(alphaid.encode(63), b"_");
    assert_eq!(alphaid.encode(64), b"ab");
    assert_eq!(alphaid.encode(20191226), b"W5bnb");
    assert_eq!(alphaid.encode(u128::max_value()), b"_____________________d");
}

#[test]
fn test_decode_basic() {
    let alphaid = AlphaId::new();
    assert_eq!(alphaid.decode(b"a"), Ok(0));
    assert_eq!(alphaid.decode(b"aab"), Ok(4096));
    assert_eq!(alphaid.decode(b"b"), Ok(1));
    assert_eq!(alphaid.decode(b"-"), Ok(62));
    assert_eq!(alphaid.decode(b"_"), Ok(63));
    assert_eq!(alphaid.decode(b"W5bnb"), Ok(20191226));
    assert_eq!(
        alphaid.decode(b"_____________________d"),
        Ok(u128::max_value())
    );
}

#[test]
fn test_decode_unexpected_char() {
    let alphaid = AlphaId::new();
    assert_eq!(alphaid.decode(b"]a"), Err(DecodeError::UnexpectedChar));
    assert_eq!(alphaid.decode(b"b!"), Err(DecodeError::UnexpectedChar));
    assert_eq!(alphaid.decode(b"/-"), Err(DecodeError::UnexpectedChar));
    assert_eq!(
        alphaid.decode(b"p83FPRwvWJs+"),
        Err(DecodeError::UnexpectedChar)
    );
}

#[test]
fn test_decode_overflow() {
    let alphaid = AlphaId::new();
    assert_eq!(
        alphaid.decode(b"opoasdfasdfZIUDIz1WwBXg"),
        Err(DecodeError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"xASDF_fdaORGAiXysf5aNe0"),
        Err(DecodeError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"fda_xfdsa-Pb7N_x_ZfkqFc6k"),
        Err(DecodeError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"IfdaxpqzljhIQi25kNu8MdY"),
        Err(DecodeError::Overflow)
    );
}

#[test]
fn test_encode_with_pad() {
    let alphaid = AlphaId::builder().pad(2).build();
    assert_eq!(alphaid.encode(0), b"ab");
    assert_eq!(alphaid.encode(1), b"bb");
    assert_eq!(alphaid.encode(62), b"-b");
    assert_eq!(alphaid.encode(63), b"_b");
    assert_eq!(alphaid.encode(u128::max_value()), b"_aaaaaaaaaaaaaaaaaaaae");
}

#[test]
fn test_encode_with_pad2() {
    let alphaid = AlphaId::builder().pad(5).build();
    assert_eq!(alphaid.encode(0), b"aaaab");
    assert_eq!(alphaid.encode(1), b"baaab");
    assert_eq!(alphaid.encode(62), b"-aaab");
    assert_eq!(alphaid.encode(63), b"_aaab");
    assert_eq!(alphaid.encode(u128::max_value()), b"____aaaaaaaaaaaaaaaaae");
}

#[test]
fn test_decode_with_pad() {
    let alphaid = AlphaId::builder().pad(2).build();
    assert_eq!(alphaid.decode(b"ab"), Ok(0));
    assert_eq!(alphaid.decode(b"bb"), Ok(1));
    assert_eq!(alphaid.decode(b"-b"), Ok(62));
    assert_eq!(alphaid.decode(b"aab"), Ok(4032));
    assert_eq!(
        alphaid.decode(b"_aaaaaaaaaaaaaaaaaaaae"),
        Ok(u128::max_value())
    );
}

#[test]
fn test_pad() {
    let x = AlphaId::builder().build();
    let y = AlphaId::builder().pad(2).build();
    let z = AlphaId::builder().pad(5).build();

    assert_eq!(x.encode(64), y.encode(0));
    assert_eq!(x.encode(64u128.pow(4)), z.encode(0));
}

#[test]
#[should_panic]
fn test_invalid_pad() {
    let x = AlphaId::builder().pad(0).build();
    x.encode(20191226);
}

#[test]
#[should_panic]
fn test_duplicated_chars() {
    let chars = "abcda".as_bytes().to_vec();
    let x = AlphaId::builder().chars(chars).build();

    x.encode(20191226);
}