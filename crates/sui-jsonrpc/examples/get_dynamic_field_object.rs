use serde_json::json;
use sui_jsonrpc::api::IndexerApiClient as _;
use sui_jsonrpc::client::SuiClientBuilder;
use sui_jsonrpc::msgs::DynamicFieldName;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = SuiClientBuilder::default().build_testnet().await?;

    let df = client
        .http()
        .get_dynamic_field_object(
            "0xe4a1c0bfc53a7c2941a433a9a681c942327278b402878e0c45280eecd098c3d1".parse()?,
            DynamicFieldName {
                type_: "0xb7adfc0868f43477d72e4e3a6f0892149d86fbf557334e8d74fabaea314faaae::keys::MarketVault".parse()?,
                value: json!({"dummy_field": false}),
            },
        )
        .await?
        .into_object()?;

    println!("{df}");

    Ok(())
}
