#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/braced_struct.rs");
    t.pass("tests/tuple_struct.rs");
    t.compile_fail("tests/empty_braced_struct.rs");
    t.compile_fail("tests/empty_tuple_struct.rs");
}
