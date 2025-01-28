use af_sui_types::Version;
use cynic::QueryFragment;
use graphql_extract::extract;

use super::Error;
use crate::{schema, GraphQlClient, GraphQlResponseExt as _};

pub async fn query<C: GraphQlClient>(client: &C) -> Result<u64, Error<C::Error>> {
    let data = client
        .query::<Query, _>(())
        .await
        .map_err(Error::Client)?
        .try_into_data()?;
    extract!(data => {
        epoch? {
            epoch_id
        }
    });
    Ok(epoch_id)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn init_gql_output() {
    use cynic::QueryBuilder as _;
    let operation = Query::build(());
    insta::assert_snapshot!(operation.query, @r###"
    query Query {
      epoch {
        epochId
      }
    }
    "###);
}

#[derive(QueryFragment, Clone, Debug)]
#[cynic(graphql_type = "Query")]
struct Query {
    epoch: Option<Epoch>,
}

#[derive(QueryFragment, Clone, Debug)]
struct Epoch {
    epoch_id: Version,
}
