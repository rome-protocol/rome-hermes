use sui_jsonrpc::api::ReadApiClient;
use sui_jsonrpc::client::SuiClientBuilder;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = SuiClientBuilder::default().build_testnet().await?;

    let id = client.http().get_chain_identifier().await?;

    println!("{id}");

    Ok(())
}
