# enum-procs

Rust macros for enums

## Usage

```rust
use enum_procs::PartialEqVariant;

#[derive(PartialEqVariant)]
enum EnumProc {
    VariantWithValue(bool),
    AnotherVariantWithValue(bool),
}

assert!(EnumProc::VariantWithValue(true) == EnumProc::VariantWithValue(false));
assert!(EnumProc::VariantWithValue(false) == EnumProc::VariantWithValue(true));

assert!(EnumProc::VariantWithValue(true) != EnumProc::AnotherVariantWithValue(false));
assert!(EnumProc::VariantWithValue(false) != EnumProc::AnotherVariantWithValue(true));
```
