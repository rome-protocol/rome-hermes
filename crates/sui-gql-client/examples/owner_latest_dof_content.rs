use color_eyre::Result;
use af_sui_types::TypeTag;
use sui_gql_client::queries::outputs::RawMoveValue;
use sui_gql_client::queries::GraphQlClientExt as _;
use sui_gql_client::reqwest::ReqwestClient;

const SUI_GRAPHQL_SERVER_URL: &str = "https://sui-testnet.mystenlabs.com/graphql";
const STAKED_SUI_VAULT_OBJECT_ID: &str = "0xe498a8c07ec62200c519a0092eda233abdab879e8f332c11bdc1819eb7b12fbb";
const VERSION: u64 = 0;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let client = ReqwestClient::new(
        reqwest::Client::default(),
        SUI_GRAPHQL_SERVER_URL.to_owned(),
    );

    let dof_name = RawMoveValue {
        type_: TypeTag::U64,
        bcs: bcs::to_bytes(&VERSION)?,
    };

    let result = client
        .owner_dof_content(STAKED_SUI_VAULT_OBJECT_ID.parse()?, dof_name, None)
        .await?;
    println!("{result:?}");
    Ok(())
}
