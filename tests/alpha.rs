use alphaid::{AlphaId, AlphaIdError};
#[test]
fn test_encode_basic() {
    let alphaid = AlphaId::new();
    assert_eq!(alphaid.encode(0), Ok(b"a".to_vec()));
    assert_eq!(alphaid.encode(1), Ok(b"b".to_vec()));
    assert_eq!(alphaid.encode(62), Ok(b"-".to_vec()));
    assert_eq!(alphaid.encode(63), Ok(b"_".to_vec()));
    assert_eq!(alphaid.encode(64), Ok(b"ab".to_vec()));
    assert_eq!(
        alphaid.encode(u128::max_value()),
        Ok(b"_____________________d".to_vec())
    );
}

#[test]
fn test_decode_basic() {
    let alphaid = AlphaId::new();
    assert_eq!(alphaid.decode(b"a"), Ok(0));
    assert_eq!(alphaid.decode(b"aab"), Ok(4096));
    assert_eq!(alphaid.decode(b"b"), Ok(1));
    assert_eq!(alphaid.decode(b"-"), Ok(62));
    assert_eq!(alphaid.decode(b"_"), Ok(63));
    assert_eq!(
        alphaid.decode(b"_____________________d"),
        Ok(u128::max_value())
    );
}

#[test]
fn test_decode_unexpected_char() {
    let alphaid: AlphaId = AlphaId::new();
    assert_eq!(alphaid.decode(b"]a"), Err(AlphaIdError::UnexpectedChar));
    assert_eq!(alphaid.decode(b"b!"), Err(AlphaIdError::UnexpectedChar));
    assert_eq!(alphaid.decode(b"/-"), Err(AlphaIdError::UnexpectedChar));
    assert_eq!(
        alphaid.decode(b"p83FPRwvWJs+"),
        Err(AlphaIdError::UnexpectedChar)
    );
}

#[test]
fn test_decode_overflow() {
    let alphaid = AlphaId::<u128>::new();
    assert_eq!(
        alphaid.decode(b"opoasdfasdfZIUDIz1WwBXg"),
        Err(AlphaIdError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"xASDF_fdaORGAiXysf5aNe0"),
        Err(AlphaIdError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"fda_xfdsa-Pb7N_x_ZfkqFc6k"),
        Err(AlphaIdError::Overflow)
    );
    assert_eq!(
        alphaid.decode(b"IfdaxpqzljhIQi25kNu8MdY"),
        Err(AlphaIdError::Overflow)
    );
}

#[test]
fn test_encode_with_pad() {
    let alphaid = AlphaId::builder().pad(2).build();
    assert_eq!(alphaid.encode(0), Ok(b"ab".to_vec()));
    assert_eq!(alphaid.encode(1), Ok(b"bb".to_vec()));
    assert_eq!(alphaid.encode(62), Ok(b"-b".to_vec()));
    assert_eq!(alphaid.encode(63), Ok(b"_b".to_vec()));
    assert_eq!(
        alphaid.encode(u128::max_value()),
        Ok(b"_aaaaaaaaaaaaaaaaaaaae".to_vec())
    );
}

#[test]
fn test_encode_with_pad2() {
    let alphaid = AlphaId::builder().pad(5).build();
    assert_eq!(alphaid.encode(0), Ok(b"aaaab".to_vec()));
    assert_eq!(alphaid.encode(1), Ok(b"baaab".to_vec()));
    assert_eq!(alphaid.encode(62), Ok(b"-aaab".to_vec()));
    assert_eq!(alphaid.encode(63), Ok(b"_aaab".to_vec()));
    assert_eq!(
        alphaid.encode(u128::max_value()),
        Ok(b"____aaaaaaaaaaaaaaaaae".to_vec())
    );
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
