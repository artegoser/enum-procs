use enum_procs::AutoFrom;

#[test]
fn partial_eq_variant() {
    #[derive(AutoFrom, Debug, PartialEq, Eq)]
    enum Test {
        Bool(bool),
        Text(String),
    }

    assert_eq!(Test::from(true), Test::Bool(true));
    assert_eq!(Test::from("Test"), Test::Text("Test".to_owned()));
}
