use super::Error;
use crate::{GraphQlClient, GraphQlResponseExt, schema};

pub async fn query<C: GraphQlClient>(
    client: &C,
    type_: impl AsRef<str>,
) -> Result<CoinMetadataResponse, Error<C::Error>> {
    let data = client
        .query::<Query, _>(QueryVariables {
            coin_type: type_.as_ref(),
        })
        .await
        .map_err(Error::Client)?
        .try_into_data()?;
    graphql_extract::extract!(data => {
        coin_metadata? {
            decimals?
            name
            symbol
        }
    });
    Ok(CoinMetadataResponse {
        decimals: decimals as u8,
        name,
        symbol,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() {
    use cynic::QueryBuilder as _;

    let vars = QueryVariables {
        coin_type: "0x2::sui::Sui",
    };
    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($coinType: String!) {
      coinMetadata(coinType: $coinType) {
        decimals
        name
        symbol
      }
    }
    "###);
}

#[derive(cynic::QueryVariables, Debug)]
struct QueryVariables<'a> {
    coin_type: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "QueryVariables")]
struct Query {
    #[arguments(coinType: $coin_type)]
    coin_metadata: Option<CoinMetadata>,
}

#[derive(cynic::QueryFragment, Debug)]
struct CoinMetadata {
    decimals: Option<i32>,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Debug)]
pub struct CoinMetadataResponse {
    pub decimals: u8,
    /// Full, official name of the token. Equivalent to CCXT's currency name.
    pub name: Option<String>,
    /// The token's identifying abbreviation. Equivalent to CCXT's currency code.
    pub symbol: Option<String>,
}
