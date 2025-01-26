//! Experimental abstractions to help create stream queries.
//!
//! Should be private until we iron out the api.
use std::future::Future;

use futures::Stream;

use super::fragments::PageInfoForward;
use super::QResult;
use crate::GraphQlClient;

pub(super) type PageResult<Iter, Client> = QResult<(PageInfoForward, Iter), Client>;

pub(super) fn forward<'a, 'b, Client, Vars, Req, Fut, Iter, T>(
    client: &'a Client,
    mut vars: Vars,
    mut request: Req,
) -> impl Stream<Item = QResult<T, Client>> + 'a
where
    Client: GraphQlClient,
    Vars: 'b + UpdatePageInfo + Clone,
    Req: 'b + FnMut(&'a Client, Vars) -> Fut,
    Fut: 'a + Future<Output = PageResult<Iter, Client>>,
    Iter: Iterator<Item = QResult<T, Client>>,
    T: 'static,
    'b: 'a,
{
    async_stream::try_stream! {
        let mut has_next_page = true;
        while has_next_page {
            let (page_info, objects) = request(client, vars.clone()).await?;

            vars.update_page_info(&page_info);
            has_next_page = page_info.has_next_page;

            for value in objects {
                yield value?;
            }
        }
    }
}

pub(super) trait UpdatePageInfo {
    fn update_page_info(&mut self, info: &PageInfoForward);
}
