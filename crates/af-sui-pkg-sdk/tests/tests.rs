#[test]
fn build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/legacy_move.rs");
    t.pass("tests/tuple_struct.rs");
    t.pass("tests/visibility_modifiers.rs");
    t.pass("tests/option_field.rs");
}

use af_sui_pkg_sdk::sui_pkg_sdk;

sui_pkg_sdk!(package {
    module dummy {
        struct Dummy {
            option: Option<u64>,
        }
    }
});

#[test]
fn display() {
    let none = dummy::Dummy::new(None);
    insta::assert_snapshot!(none, @r###"
    ╭────────┬──────╮
    │ Dummy         │
    ├────────┼──────┤
    │ option │ None │
    ╰────────┴──────╯
    "###);
    let some = dummy::Dummy::new(Some(1));
    insta::assert_snapshot!(some, @r###"
    ╭────────┬───╮
    │ Dummy      │
    ├────────┼───┤
    │ option │ 1 │
    ╰────────┴───╯
    "###);
}
