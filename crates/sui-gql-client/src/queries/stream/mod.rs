//! Experimental abstractions to help create stream queries.
//!
//! Should be private until we iron out the api.
use std::future::Future;

use futures::Stream;

use super::fragments::PageInfo;
use super::Result;
use crate::GraphQlClient;

/// Helper for paginating queries forward.
///
/// # Arguments
/// - `client`
/// - `vars`: [`cynic::QueryVariables`] fragment; must implement [`UpdatePageInfo`]
/// - `request`: async function that maps `(client, vars) -> Page<Iter>`, where `Iter` is an
///   [`Iterator`] over items of a **single** page's results
pub(super) fn forward<'a, 'b, Client, Vars, Req, Fut, Iter, T>(
    client: &'a Client,
    mut vars: Vars,
    mut request: Req,
) -> impl Stream<Item = Result<T, Client>> + 'a
where
    Client: GraphQlClient,
    Vars: 'b + UpdatePageInfo + Clone,
    Req: 'b + FnMut(&'a Client, Vars) -> Fut,
    Fut: 'a + Future<Output = Result<Page<Iter>, Client>>,
    Iter: Iterator<Item = Result<T, Client>>,
    T: 'static,
    'b: 'a,
{
    async_stream::try_stream! {
        let mut has_next_page = true;
        while has_next_page {
            let page = request(client, vars.clone()).await?;

            vars.update_page_info(&page.info);
            has_next_page = page.info.has_next_page;

            for value in page.data {
                yield value?;
            }
        }
    }
}

pub(super) struct Page<T> {
    pub(super) info: PageInfo,
    pub(super) data: T,
}

impl<T> Page<T> {
    pub(super) fn new(page_info: impl Into<PageInfo>, data: T) -> Self {
        Self {
            info: page_info.into(),
            data,
        }
    }
}

pub(super) trait UpdatePageInfo {
    fn update_page_info(&mut self, info: &PageInfo);
}
