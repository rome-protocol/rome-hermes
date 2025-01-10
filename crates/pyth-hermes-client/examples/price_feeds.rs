use clap::Parser;
use pyth_hermes_client::{AssetType, PythClient};

#[derive(Parser)]
struct Args {
    #[arg(default_value = "bitcoin")]
    query: String,

    #[arg(long)]
    asset_type: Option<AssetType>,

    #[arg(long, default_value = "https://hermes-beta.pyth.network")]
    endpoint: url::Url,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Args {
        query,
        asset_type,
        endpoint,
    } = Args::parse();

    let client = PythClient::new(endpoint);

    let response = client.price_feeds(query, asset_type).await?;
    for info in response {
        println!("{info:#?}");
    }
    Ok(())
}
