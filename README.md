
# ALPHAID

Generate Youtube-Like IDs with Rust

[![Build Status](https://travis-ci.com/importcjj/alphaid.svg?token=ZZrg3rRkUA8NUGrjEsU9&branch=master)](https://travis-ci.com/importcjj/alphaid) [![crates.io](https://img.shields.io/badge/crates.io-latest-%23dea584)](https://crates.io/crates/alphaid)


## Basic Usage

```rust
use alphaid::AlphaId;

let alphaid = AlphaId::new();
assert_eq!(alphaid.encode(1350997667), b"90F7qb");
assert_eq!(alphaid.decode(b"90F7qb"), Ok(1350997667));
```

## Padding
Specifies the minimum length of the encoded result.

```rust
use alphaid::AlphaId;

let alphaid = AlphaId::new();
assert_eq!(alphaid.encode(0), b"a");
assert_eq!(alphaid.decode(b"a"), Ok(0));


let alphaid = AlphaId::builder().pad(5).build();
assert_eq!(alphaid.encode(0), b"aaaab");
assert_eq!(alphaid.decode(b"aaaab"), Ok(0));
```

## Charaters set
Sets the characters set. Default to `abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-_`

```rust
let alphaid = AlphaId::builder().pad(2)
    .chars("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().to_vec())
    .build();
assert_eq!(alphaid.encode(0), b"AB");
assert_eq!(alphaid.decode(b"AB"), Ok(0));
```


## Reference

[Create Youtube-Like IDs](https://kvz.io/create-short-ids-with-php-like-youtube-or-tinyurl.html)