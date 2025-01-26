use af_sui_types::{Address, Object, ObjectId};
use futures::Stream;
use itertools::Itertools as _;
use sui_gql_schema::scalars::Base64Bcs;

use super::fragments::{ObjectFilterV2, PageInfoForward};
use crate::queries::Error;
use crate::{extract, missing_data, scalars, schema, GraphQlClient, GraphQlResponseExt};

pub(super) fn query<C: GraphQlClient>(
    client: &C,
    owner: Option<Address>,
    type_: Option<String>,
    page_size: Option<u32>,
) -> impl Stream<Item = Result<Object, Error<C::Error>>> + '_ {
    let filter = ObjectFilterV2 {
        owner,
        type_,
        ..Default::default()
    };
    let vars = Variables {
        after: None,
        first: page_size.map(|v| v.try_into().unwrap_or(i32::MAX)),
        filter: Some(filter),
    };
    super::stream::forward(client, vars, request)
}

async fn request<C: GraphQlClient>(
    client: &C,
    vars: Variables<'_>,
) -> Result<
    (
        PageInfoForward,
        impl Iterator<Item = Result<Object, Error<C::Error>>> + 'static,
    ),
    Error<C::Error>,
> {
    let response = client
        .query::<Query, _>(vars)
        .await
        .map_err(Error::Client)?;
    let data = response.try_into_data()?;

    let ObjectConnection { nodes, page_info } = extract!(data?.objects);
    let raw_objs = nodes
        .into_iter()
        .map(|ObjectGql { id, object }| object.ok_or(missing_data!("Bcs for object {id}")))
        .map_ok(Base64Bcs::into_inner);

    Ok((page_info, raw_objs))
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables<'a> {
    filter: Option<ObjectFilterV2<'a>>,
    after: Option<String>,
    first: Option<i32>,
}

impl super::stream::UpdatePageInfo for Variables<'_> {
    fn update_page_info(&mut self, info: &PageInfoForward) {
        self.after.clone_from(&info.end_cursor)
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
    object: Option<scalars::Base64Bcs<Object>>,
}

#[cfg(test)]
#[test]
fn gql_output() -> color_eyre::Result<()> {
    use cynic::QueryBuilder as _;

    let vars = Variables {
        filter: None,
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
