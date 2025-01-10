use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clap::Parser;
use pyth_hermes_client::{EncodingType, PriceIdInput, PythClient};

#[derive(Parser)]
struct Args {
    #[arg(default_value = "0x50c67b3fd225db8912a424dd4baed60ffdde625ed2feaaf283724f9608fea266")]
    ids: Vec<PriceIdInput>,

    #[arg(long)]
    publish_time: Option<u64>,

    #[arg(long)]
    encoding: Option<EncodingType>,

    #[arg(long)]
    parsed: Option<bool>,

    #[arg(long, default_value = "https://hermes-beta.pyth.network")]
    endpoint: url::Url,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Args {
        ids,
        publish_time,
        encoding,
        parsed,
        endpoint,
    } = Args::parse();

    let client = PythClient::new(endpoint);

    let publish_time = publish_time
        .or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|d| d - Duration::from_secs(5))
                .as_ref()
                .map(Duration::as_secs)
        })
        .expect("Went back in time");
    let response = client
        .price_update(publish_time, ids, encoding, parsed)
        .await?;
    println!("{response:#?}");
    Ok(())
}
