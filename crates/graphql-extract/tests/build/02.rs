struct Query {
    address: Option<Address2>,
    object: Option<Object>,
}

struct Address2 {
    address: String,
}

struct Object {
    version: u64,
    dynamic_field: Option<DynamicField>,
    dynamic_fields: DynamicFieldConnection,
}

struct DynamicFieldConnection {
    nodes: Vec<DynamicField>,
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
            dynamic_fields {
                nodes[][] {
                    value? {
                        ... on MoveValue {
                            type_
                            bcs
                        }
                    }
                }
            }
        }
    });
    Ok((version, nodes))
}

fn main() {}
