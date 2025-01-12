use sui_jsonrpc::api::IndexerApiClient;
use sui_jsonrpc::client::SuiClientBuilder;
use sui_jsonrpc::msgs::Page;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = SuiClientBuilder::default().build_testnet().await?;

    let Page {
        data,
        next_cursor,
        has_next_page,
    } = client
        .http()
        .get_dynamic_fields(
            "0xe4a1c0bfc53a7c2941a433a9a681c942327278b402878e0c45280eecd098c3d1".parse()?,
            None,
            None,
        )
        .await?;

    for d in data {
        println!("{d:#?}");
    }
    if !has_next_page {
        return Ok(());
    }
    if let Some(cursor) = next_cursor {
        println!("Next cursor: {cursor}");
    }

    Ok(())
}
