use af_sui_types::{ObjectId, Version};
use futures::TryStreamExt as _;

use super::fragments::{ObjectFilterV2, PageInfoForward};
use super::Error;
use crate::{extract, schema, GraphQlClient};

type Item = (ObjectId, u64, u64);

pub async fn query<C: GraphQlClient>(
    client: &C,
    package_ids: Vec<ObjectId>,
) -> Result<impl Iterator<Item = Item>, Error<C::Error>> {
    let vars = QueryVariables {
        filter: Some(ObjectFilterV2 {
            type_: None,
            owner: None,
            object_ids: Some(&package_ids),
        }),
        first: None,
        after: None,
    };

    let results: Vec<_> = super::stream::forward(client, vars, request)
        .try_collect()
        .await?;

    Ok(results.into_iter())
}

async fn request<C: GraphQlClient>(
    client: &C,
    vars: QueryVariables<'_>,
) -> super::Result<
    (
        PageInfoForward,
        impl Iterator<Item = super::Result<Item, C>>,
    ),
    C,
> {
    let query = client
        .query::<Query, _>(vars)
        .await
        .map_err(Error::Client)?;
    let ObjectConnection { nodes, page_info } = extract!(query.data?.objects);
    Ok((
        page_info,
        nodes.into_iter().map(|node| {
            let effects = extract!(node.as_move_package?.previous_transaction_block?.effects?);
            let epoch_id = extract!(effects.epoch?.epoch_id);
            let ckpt_seq = extract!(effects.checkpoint?.sequence_number);
            Ok((node.address, epoch_id, ckpt_seq))
        }),
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() {
    use cynic::QueryBuilder as _;

    let vars = QueryVariables {
        filter: None,
        first: None,
        after: None,
    };
    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($after: String, $filter: ObjectFilter, $first: Int) {
      objects(filter: $filter, first: $first, after: $after) {
        nodes {
          address
          asMovePackage {
            previousTransactionBlock {
              effects {
                epoch {
                  epochId
                }
                checkpoint {
                  sequenceNumber
                }
              }
            }
          }
        }
        pageInfo {
          hasNextPage
          endCursor
        }
      }
    }
    "###);
}

// ================================================================================

impl super::stream::UpdatePageInfo for QueryVariables<'_> {
    fn update_page_info(&mut self, info: &PageInfoForward) {
        self.after.clone_from(&info.end_cursor)
    }
}

// ================================================================================

#[derive(cynic::QueryVariables, Clone, Debug)]
struct QueryVariables<'a> {
    after: Option<String>,
    filter: Option<ObjectFilterV2<'a>>,
    first: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "QueryVariables")]
struct Query {
    #[arguments(filter: $filter, first: $first, after: $after)]
    objects: ObjectConnection,
}

#[derive(cynic::QueryFragment, Debug)]
struct ObjectConnection {
    nodes: Vec<Object>,
    page_info: PageInfoForward,
}

#[derive(cynic::QueryFragment, Debug)]
struct Object {
    address: ObjectId,
    as_move_package: Option<MovePackage>,
}

#[derive(cynic::QueryFragment, Debug)]
struct MovePackage {
    previous_transaction_block: Option<TransactionBlock>,
}

#[derive(cynic::QueryFragment, Debug)]
struct TransactionBlock {
    effects: Option<TransactionBlockEffects>,
}

#[derive(cynic::QueryFragment, Debug)]
struct TransactionBlockEffects {
    epoch: Option<Epoch>,
    checkpoint: Option<Checkpoint>,
}

#[derive(cynic::QueryFragment, Debug)]
struct Epoch {
    epoch_id: Version,
}

#[derive(cynic::QueryFragment, Debug)]
struct Checkpoint {
    sequence_number: Version,
}
