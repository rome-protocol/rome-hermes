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
        struct Int {
            option: Option<u64>,
        }
        struct Str {
            option: Option<String>,
        }
        struct Nested {
            option: Option<Str>
        }
    }
});

#[test]
fn display() {
    let none = dummy::Int::new(None);
    insta::assert_snapshot!(none, @r###"
    ╭────────┬──╮
    │ Int       │
    ├────────┼──┤
    │ option │  │
    ╰────────┴──╯
    "###);
    let some = dummy::Int::new(Some(1));
    insta::assert_snapshot!(some, @r###"
    ╭────────┬───╮
    │ Int        │
    ├────────┼───┤
    │ option │ 1 │
    ╰────────┴───╯
    "###);
    let none = dummy::Str::new(None);
    insta::assert_snapshot!(none, @r###"
    ╭────────┬──╮
    │ Str       │
    ├────────┼──┤
    │ option │  │
    ╰────────┴──╯
    "###);
    let some = dummy::Str::new(Some(String::new()));
    insta::assert_snapshot!(some, @r###"
    ╭────────┬──╮
    │ Str       │
    ├────────┼──┤
    │ option │  │
    ╰────────┴──╯
    "###);
    let nested_none = dummy::Nested::new(None);
    insta::assert_snapshot!(nested_none, @r###"
    ╭────────┬──╮
    │ Nested    │
    ├────────┼──┤
    │ option │  │
    ╰────────┴──╯
    "###);
    let nested_some = dummy::Nested::new(Some(dummy::Str::new(Some("1".into()))));
    insta::assert_snapshot!(nested_some, @r###"
    ╭────────┬────────────────╮
    │ Nested                  │
    ├────────┼────────────────┤
    │ option │ ╭────────┬───╮ │
    │        │ │ Str        │ │
    │        │ ├────────┼───┤ │
    │        │ │ option │ 1 │ │
    │        │ ╰────────┴───╯ │
    ╰────────┴────────────────╯
    "###);
}
