use af_sui_types::Version;
use cynic::QueryFragment;
use graphql_extract::extract;

use super::Error;
use crate::{GraphQlClient, GraphQlResponseExt as _, schema};

pub async fn query<C: GraphQlClient>(client: &C, epoch_id: u64) -> Result<u64, Error<C::Error>> {
    let data = client
        .query::<Query, _>(Variables { id: Some(epoch_id) })
        .await
        .map_err(Error::Client)?
        .try_into_data()?;
    Ok(extract(data)?)
}

fn extract(data: Option<Query>) -> Result<Version, &'static str> {
    extract!(data => {
        epoch? {
            checkpoints {
                nodes[] {
                    sequence_number
                }
            }
        }
    });
    let mut nodes = nodes;
    nodes.next().ok_or("Empty epoch checkpoints")?
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn init_gql_output() {
    use cynic::QueryBuilder as _;
    let operation = Query::build(Variables { id: None });
    insta::assert_snapshot!(operation.query, @r###"
    query Query($id: UInt53) {
      epoch(id: $id) {
        checkpoints(last: 1) {
          nodes {
            sequenceNumber
          }
        }
      }
    }
    "###);
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables {
    id: Option<Version>,
}

#[derive(QueryFragment, Clone, Debug)]
#[cynic(graphql_type = "Query", variables = "Variables")]
struct Query {
    #[arguments(id: $id)]
    epoch: Option<Epoch>,
}

#[derive(QueryFragment, Clone, Debug)]
struct Epoch {
    #[arguments(last: 1)]
    checkpoints: CheckpointConnection,
}

#[derive(QueryFragment, Clone, Debug)]
struct CheckpointConnection {
    nodes: Vec<Checkpoint>,
}

#[derive(QueryFragment, Clone, Debug)]
struct Checkpoint {
    sequence_number: Version,
}
