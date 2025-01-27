#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/01.rs");
    t.compile_fail("tests/build/02.rs");
    t.compile_fail("tests/build/03.rs");
    t.compile_fail("tests/build/04.rs");
    t.compile_fail("tests/build/05.rs");
    t.compile_fail("tests/build/06.rs");
    t.compile_fail("tests/build/07.rs");
}
