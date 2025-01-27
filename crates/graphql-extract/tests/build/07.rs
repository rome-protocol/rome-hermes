struct Query {
    object: Option<Object>,
}

struct Object {
    version: u64,
    dynamic_field: Option<DynamicField>,
}

struct DynamicField {
    value: Option<DynamicFieldValue>,
}

enum DynamicFieldValue {
    MoveValue(MoveValue),
    Unknown,
}

struct MoveValue {
    type_: MoveType,
    bcs: String,
}

struct MoveType {
    repr: String,
}

type Item = Result<(MoveType, String), &'static str>;

fn extract(data: Option<Query>) -> Result<(u64, impl Iterator<Item = Item>), &'static str> {
    use graphql_extract::extract;
    use DynamicFieldValue::MoveValue;

    extract!(data => {
        object? {
            version
            dynamic_field {
                value? {
                    ... on MoveValue {
                        type_
                        bcs
                    }
                    node
                }
            }
        }
    });
    Ok((version, nodes))
}

fn main() {}
