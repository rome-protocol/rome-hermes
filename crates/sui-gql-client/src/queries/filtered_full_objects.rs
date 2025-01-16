use std::collections::HashMap;

use af_sui_types::{Address, Object, ObjectId};
use futures::Stream;

use super::fragments::{ObjectFilterV2, PageInfoForward};
use crate::queries::Error;
use crate::{extract, missing_data, scalars, schema, GraphQlClient, GraphQlResponseExt, Paged};

pub(super) fn query<C: GraphQlClient>(
    client: &C,
    owner: Option<Address>,
    type_: Option<String>,
    page_size: Option<u32>,
) -> impl Stream<Item = Result<(ObjectId, Object), Error<C::Error>>> + '_ {
    async_stream::try_stream! {
        let filter = ObjectFilterV2 {
            owner,
            type_,
            ..Default::default()
        };
        let mut vars = Variables {
            after: None,
            first: page_size.map(|v| v.try_into().unwrap_or(i32::MAX)),
            filter: Some(filter),
        };
        let mut has_next_page = true;
        while has_next_page {
            let (page_info, objects) = request(client, vars.clone()).await?;

            vars.after = page_info.end_cursor.clone();
            has_next_page = page_info.has_next_page;

            for value in objects {
                yield value;
            }
        }
    }
}

async fn request<C: GraphQlClient>(
    client: &C,
    vars: Variables,
) -> Result<
    (
        PageInfoForward,
        impl Iterator<Item = (ObjectId, Object)> + 'static,
    ),
    Error<C::Error>,
> {
    let response = client
        .query::<Query, _>(vars)
        .await
        .map_err(Error::Client)?;
    let data = response.try_into_data()?;

    let ObjectConnection { nodes, page_info } = extract!(data?.objects);
    let mut raw_objs = HashMap::new();
    for ObjectGql { id, object } in nodes.into_iter() {
        let wrapped = object.ok_or(missing_data!("Bcs for object {id}"))?;
        raw_objs.insert(id, wrapped.into_inner());
    }
    Ok((page_info, raw_objs.into_iter()))
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables {
    filter: Option<ObjectFilterV2>,
    after: Option<String>,
    first: Option<i32>,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(variables = "Variables")]
struct Query {
    #[arguments(filter: $filter, first: $first, after: $after)]
    objects: ObjectConnection,
}

impl Paged for Query {
    type Input = Variables;

    type NextInput = Variables;

    type NextPage = Self;

    fn next_variables(&self, mut prev_vars: Self::Input) -> Option<Self::NextInput> {
        let PageInfoForward {
            has_next_page,
            end_cursor,
        } = &self.objects.page_info;
        if *has_next_page {
            prev_vars.after.clone_from(end_cursor);
            Some(prev_vars)
        } else {
            None
        }
    }
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
