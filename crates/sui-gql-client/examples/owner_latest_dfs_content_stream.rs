use color_eyre::Result;
use futures::TryStreamExt as _;
use sui_gql_client::queries::GraphQlClientExt as _;
use sui_gql_client::reqwest::ReqwestClient;

const SUI_GRAPHQL_SERVER_URL: &str = "https://sui-testnet.mystenlabs.com/graphql";
const BTC_BIDS_MAP: &str = "0x5fb03a8666f451cef89758c9ad3247230436e46ee71544430124130558c91d5a";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let client = ReqwestClient::new(
        reqwest::Client::default(),
        SUI_GRAPHQL_SERVER_URL.to_owned(),
    );

    tokio::pin!(
        let stream = client.owner_df_contents_stream(BTC_BIDS_MAP.parse()?, None, None).await;
    );

    let mut count = 0;
    while let Some((name, value)) = stream.try_next().await? {
        count += 1;
        println!("Name: {name:?}, Value: {value:?}");
    }
    println!("Objects count: {count}");

    Ok(())
}
