use sui_jsonrpc::api::ReadApiClient;
use sui_jsonrpc::client::SuiClientBuilder;
use sui_jsonrpc::msgs::SuiObjectDataOptions;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = SuiClientBuilder::default().build_testnet().await?;

    let object = client
        .http()
        .get_object(
            "0xe4a1c0bfc53a7c2941a433a9a681c942327278b402878e0c45280eecd098c3d1".parse()?,
            Some(SuiObjectDataOptions::full_content()),
        )
        .await?
        .into_object()?;

    println!("{object}");

    Ok(())
}
