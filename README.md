
# ALPHAID

Generate Youtube-Like IDs with Rust

[![Build Status](https://travis-ci.com/importcjj/alphaid.svg?token=ZZrg3rRkUA8NUGrjEsU9&branch=master)](https://travis-ci.com/importcjj/alphaid) [![crates.io](https://img.shields.io/badge/crates.io-latest-%23dea584)](https://crates.io/crates/alphaid)

## Example

```rust
use alphaid::AlphaId;

let alphaid = AlphaId::new();
assert_eq!(alphaid.encode(1350997667), b"90F7qb");

assert_eq!(alphaid.decode(b"90F7qb"), Ok(1350997667));

let alphaid = AlphaId::builder().pad(2).build();
assert_eq!(alphaid.encode(0), b"ab");
assert_eq!(alphaid.decode(b"ab"), Ok(0));

let alphaid = AlphaId::builder().pad(2)
    .chars("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().to_vec())
    .build();
assert_eq!(alphaid.encode(0), b"AB");
assert_eq!(alphaid.decode(b"AB"), Ok(0));
```


## Reference

[Create Youtube-Like IDs](https://kvz.io/create-short-ids-with-php-like-youtube-or-tinyurl.html)