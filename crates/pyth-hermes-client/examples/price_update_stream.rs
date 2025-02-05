use clap::Parser;
use futures::{StreamExt as _, TryStreamExt as _};
use pyth_hermes_client::{EncodingType, PriceIdInput, PythClient};

#[derive(Parser)]
struct Args {
    #[arg(default_value = "0x50c67b3fd225db8912a424dd4baed60ffdde625ed2feaaf283724f9608fea266")]
    ids: Vec<PriceIdInput>,

    #[arg(long)]
    encoding: Option<EncodingType>,

    #[arg(long)]
    parsed: Option<bool>,

    #[arg(long)]
    allow_unordered: Option<bool>,

    #[arg(long)]
    benchmarks_only: Option<bool>,

    #[arg(long)]
    drop_after: Option<usize>,

    #[arg(long, default_value = "https://hermes-beta.pyth.network")]
    endpoint: url::Url,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Args {
        ids,
        encoding,
        parsed,
        allow_unordered,
        benchmarks_only,
        drop_after,
        endpoint,
    } = Args::parse();

    let client = PythClient::new(endpoint);

    let mut stream = client
        .stream_price_updates(ids, encoding, parsed, allow_unordered, benchmarks_only)
        .await?
        .take(drop_after.unwrap_or(usize::MAX));

    while let Some(event) = stream.try_next().await? {
        let json = serde_json::to_value(event)?;
        println!("{json:#}");
    }
    Ok(())
}
