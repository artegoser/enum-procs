use enum_procs::PartialEqExceptLast;

#[test]
fn partial_eq_variant() {
    #[derive(PartialEqExceptLast)]
    enum EnumProc {
        VariantWithValue(bool, bool),
        Var(bool),
    }

    assert!(EnumProc::VariantWithValue(true, true) != EnumProc::VariantWithValue(false, false));
    assert!(EnumProc::VariantWithValue(true, false) == EnumProc::VariantWithValue(true, true));

    assert!(EnumProc::Var(true) == EnumProc::Var(false));
    assert!(EnumProc::Var(true) == EnumProc::Var(true));
}
