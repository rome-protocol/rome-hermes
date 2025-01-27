#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/braced_struct.rs");
}
