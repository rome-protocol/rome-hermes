struct Query {
    address: Option<Address2>,
}

struct Address2 {
    address: String,
}

type Item = Result<(MoveType, String), &'static str>;

fn extract(data: Option<Query>) -> Result<(u64, impl Iterator<Item = Item>), &'static str> {
    use graphql_extract::extract;
    use DynamicFieldValue::MoveValue;

    extract!(data => {
        address??
    });
    Ok((version, nodes))
}

fn main() {}
