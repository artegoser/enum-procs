use enum_procs::PartialEqVariant;

#[test]
fn partial_eq_variant() {
    #[derive(PartialEqVariant, Eq)]
    enum EnumProc {
        VariantWithValue(bool),
        AnotherVariantWithValue(bool),
        VariantWithoutValue,
        AnotherVariantWithoutValue,
    }

    assert!(EnumProc::VariantWithValue(true) == EnumProc::VariantWithValue(false));
    assert!(EnumProc::VariantWithValue(false) == EnumProc::VariantWithValue(true));

    assert!(EnumProc::VariantWithValue(true) != EnumProc::VariantWithoutValue);
    assert!(EnumProc::VariantWithValue(true) != EnumProc::AnotherVariantWithValue(true));

    assert!(EnumProc::VariantWithValue(true) != EnumProc::AnotherVariantWithoutValue);
}
