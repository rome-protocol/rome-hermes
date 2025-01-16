use std::str::FromStr;

use af_sui_types::TypeTag;
use clap::Parser;
use color_eyre::Result;
use sui_gql_client::queries::GraphQlClientExt as _;
use sui_gql_client::reqwest::ReqwestClient;

// Execute with
// cargo run --example filtered_full_objects

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "https://sui-testnet.mystenlabs.com/graphql")]
    rpc: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let Args { rpc } = Args::parse();

    let owner = Some("0x62ddcde37c6321d8f4bf174c35aeff6bc50cdf490e74f6e6c66d74ca2fe9ac4e".parse()?);
    let type_ = Some(TypeTag::from_str(
        "0x9f992cc2430a1f442ca7a5ca7638169f5d5c00e0ebc3977a65e9ac6e497fe5ef::staked_wal::StakedWal",
    )?);
    let client = ReqwestClient::new(reqwest::Client::default(), rpc);
    let objects = client.filtered_full_objects(owner, type_, None).await?;
    for (id, obj) in objects {
        println!("Id: {id}, Object: {obj:?}");
    }
    Ok(())
}
