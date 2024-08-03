# enum-procs

Useful enum macros

[![crates.io](https://img.shields.io/crates/v/enum-procs.svg)](https://crates.io/crates/enum-procs)

## AutoFrom

Derive macro generating an impl of the trait `From` for all types
inside tuple variants with one type

```rust
use enum_procs::AutoFrom;

#[derive(AutoFrom, Debug, PartialEq, Eq)]
enum Test {
    Bool(bool),
    Text(String),
}

assert_eq!(Test::from(true), Test::Bool(true));
assert_eq!(Test::from("Test"), Test::Text("Test".to_owned()));
```


## PartialEqVariant

Derive macro generating an impl of the trait `PartialEq` that compare enum only by variant

```rust
use enum_procs::PartialEqVariant;

#[derive(PartialEqVariant, Eq)]
enum EnumProc {
    VariantWithValue(bool),
    AnotherVariantWithValue(bool),
}

assert!(EnumProc::VariantWithValue(true) == EnumProc::VariantWithValue(false));
assert!(EnumProc::VariantWithValue(false) == EnumProc::VariantWithValue(true));
```
