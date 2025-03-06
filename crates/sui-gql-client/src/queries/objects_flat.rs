use cynic::QueryFragment;

use super::fragments::{ObjectFilterV2, PageInfoForward};
use crate::schema;

/// Paged object data where the `Object` fragment does take any parameters.
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(variables = "Variables")]
pub(crate) struct Query<Object>
where
    Object: QueryFragment<SchemaType = schema::Object, VariablesFields = ()>,
{
    #[arguments(filter: $filter, first: $first, after: $after)]
    pub objects: ObjectConnection<Object>,
}

impl<Object> Query<Object> where
    Object: QueryFragment<SchemaType = schema::Object, VariablesFields = ()>
{
}

// =============================================================================
//  Variables
// =============================================================================

#[derive(cynic::QueryVariables, Clone, Debug)]
pub(crate) struct Variables<'a> {
    pub filter: Option<ObjectFilterV2<'a>>,
    pub after: Option<String>,
    pub first: Option<i32>,
}

impl super::stream::UpdatePageInfo for Variables<'_> {
    fn update_page_info(&mut self, info: &super::fragments::PageInfo) {
        self.after.clone_from(&info.end_cursor)
    }
}

// =============================================================================
//  Inner query fragments
// =============================================================================

/// `ObjectConnection` where the `Object` fragment does take any parameters.
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(graphql_type = "ObjectConnection")]
pub(crate) struct ObjectConnection<Object>
where
    Object: QueryFragment<SchemaType = schema::Object, VariablesFields = ()>,
{
    pub nodes: Vec<Object>,
    pub page_info: PageInfoForward,
}
