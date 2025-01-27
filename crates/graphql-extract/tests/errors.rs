#![expect(dead_code, reason = "Dummy query types")]

#[derive(Debug)]
struct Query {
    object: Option<Object>,
}

#[derive(Debug)]
struct Object {
    dynamic_field: Option<DynamicField>,
}

#[derive(Debug)]
struct DynamicField {
    value: Option<DynamicFieldValue>,
}

#[derive(Debug)]
enum DynamicFieldValue {
    MoveValue(MoveValue),
    Unknown,
}

#[derive(Debug)]
struct MoveValue {
    type_: MoveType,
    bcs: Option<String>,
}

#[derive(Debug)]
struct MoveType {
    repr: String,
}

fn extract(data: Option<Query>) -> Result<(MoveType, String), &'static str> {
    use graphql_extract::extract;
    use DynamicFieldValue::MoveValue;

    extract!(data => {
        object? {
            dynamic_field? {
                value? {
                    ... on MoveValue {
                        type_
                        bcs?
                    }
                }
            }
        }
    });
    Ok((type_, bcs))
}

#[test]
fn missing_value() {
    let data = Some(Query {
        object: Some(Object {
            dynamic_field: Some(DynamicField { value: None }),
        }),
    });

    let err = extract(data).expect_err("Not Ok");
    insta::assert_snapshot!(err, @"data -> object -> dynamic_field -> value is null");
}

#[test]
fn missing_bcs() {
    let data = Some(Query {
        object: Some(Object {
            dynamic_field: Some(DynamicField {
                value: Some(DynamicFieldValue::MoveValue(MoveValue {
                    type_: MoveType {
                        repr: "type_name".into(),
                    },
                    bcs: None,
                })),
            }),
        }),
    });

    let err = extract(data).expect_err("Not Ok");
    insta::assert_snapshot!(err, @"data -> object -> dynamic_field -> value ... on MoveValue -> bcs is null");
}

#[test]
fn wrong_variant() {
    let data = Some(Query {
        object: Some(Object {
            dynamic_field: Some(DynamicField {
                value: Some(DynamicFieldValue::Unknown),
            }),
        }),
    });

    let err = extract(data).expect_err("Not Ok");
    insta::assert_snapshot!(err, @"data -> object -> dynamic_field -> value ... on MoveValue is null");
}
