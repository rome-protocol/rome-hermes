#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/legacy_move.rs");
    t.pass("tests/visibility_modifiers.rs");
    t.pass("tests/tuple_struct.rs");
}
