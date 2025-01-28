use std::collections::HashMap;

use af_sui_types::{Object, ObjectId};
use futures::{StreamExt as _, TryStreamExt as _};
use graphql_extract::extract;
use itertools::{Either, Itertools as _};
use sui_gql_schema::scalars::Base64Bcs;

use super::fragments::{ObjectFilter, ObjectKey, PageInfo, PageInfoForward};
use super::stream;
use crate::queries::Error;
use crate::{missing_data, schema, GraphQlClient, GraphQlResponseExt as _};

pub(super) async fn query<C: GraphQlClient>(
    client: &C,
    objects: impl IntoIterator<Item = (ObjectId, Option<u64>)> + Send,
    page_size: Option<u32>,
) -> Result<HashMap<ObjectId, Object>, Error<C::Error>> {
    // To keep track of all ids requested.
    let mut requested = vec![];

    let (object_ids, object_keys) = objects
        .into_iter()
        .inspect(|(id, _)| requested.push(*id))
        .partition_map(|(id, v)| {
            v.map_or(Either::Left(id), |n| {
                Either::Right(ObjectKey {
                    object_id: id,
                    version: n,
                })
            })
        });

    #[expect(
        deprecated,
        reason = "TODO: build query from scratch with new ObjectFilter and Query.multiGetObjects"
    )]
    let filter = ObjectFilter {
        object_ids: Some(object_ids),
        object_keys: Some(object_keys),
        ..Default::default()
    };
    let vars = Variables {
        after: None,
        first: page_size.map(|v| v.try_into().unwrap_or(i32::MAX)),
        filter: Some(filter),
    };

    let raw_objs: HashMap<_, _> = stream::forward(client, vars, request)
        .map(|r| -> super::Result<_, C> {
            let (id, obj) = r?;
            Ok((id, obj.ok_or_else(|| missing_data!("BCS for {id}"))?))
        })
        .try_collect()
        .await?;

    // Ensure all requested objects were returned
    for id in requested {
        raw_objs
            .contains_key(&id)
            .then_some(())
            .ok_or(missing_data!("Object {id}"))?;
    }

    Ok(raw_objs)
}

async fn request<C: GraphQlClient>(
    client: &C,
    vars: Variables,
) -> super::Result<
    stream::Page<impl Iterator<Item = super::Result<(ObjectId, Option<Object>), C>> + 'static>,
    C,
> {
    let data = client
        .query::<Query, _>(vars)
        .await
        .map_err(Error::Client)?
        .try_into_data()?;

    extract!(data => {
        objects {
            nodes[] {
                id
                object
            }
            page_info
        }
    });
    Ok(stream::Page::new(
        page_info,
        nodes.map(|r| -> super::Result<_, C> {
            let (id, obj) = r?;
            Ok((id, obj.map(Base64Bcs::into_inner)))
        }),
    ))
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables {
    filter: Option<ObjectFilter>,
    after: Option<String>,
    first: Option<i32>,
}

impl stream::UpdatePageInfo for Variables {
    fn update_page_info(&mut self, info: &PageInfo) {
        self.after.clone_from(&info.end_cursor);
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(variables = "Variables")]
struct Query {
    #[arguments(filter: $filter, first: $first, after: $after)]
    objects: ObjectConnection,
}

// =============================================================================
//  Inner query fragments
// =============================================================================

/// `ObjectConnection` where the `Object` fragment does take any parameters.
#[derive(cynic::QueryFragment, Clone, Debug)]
struct ObjectConnection {
    nodes: Vec<ObjectGql>,
    page_info: PageInfoForward,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Object")]
struct ObjectGql {
    #[cynic(rename = "address")]
    id: ObjectId,
    #[cynic(rename = "bcs")]
    object: Option<Base64Bcs<Object>>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() -> color_eyre::Result<()> {
    use cynic::QueryBuilder as _;

    let vars = Variables {
        filter: Some(ObjectFilter {
            object_ids: Some(vec![
                "0x4264c07a42f9d002c1244e43a1f0fa21c49e4a25c7202c597b8476ef6bb57113".parse()?,
                "0x60d1a85f81172a7418206f4b16e1e07e40c91cf58783f63f18a25efc81442dcb".parse()?,
            ]),
            ..Default::default()
        }),
        after: None,
        first: None,
    };

    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($filter: ObjectFilter, $after: String, $first: Int) {
      objects(filter: $filter, first: $first, after: $after) {
        nodes {
          address
          bcs
        }
        pageInfo {
          hasNextPage
          endCursor
        }
      }
    }
    "###);
    Ok(())
}
