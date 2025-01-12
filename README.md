# Aftermath Rust SDK

Crates for interacting with Aftermath's services and the Sui network. It aims to be light-weight and a complement to Sui's [sdk](https://github.com/MystenLabs/sui).

## Crates summary

See the `description` in the manifest file (`Cargo.toml`) and the `README.md` of each crate for a brief summary. For beginners, I recommend briefly checking out these crates in order:
- `af-sui-types`
- `af-move-type`
- `af-sui-pkg-sdk`
- `move-stdlib-sdk`
- `sui-jsonrpc`
- `sui-gql-client`

You'll quickly figure out that a lot of crates build on top of `af-sui-pkg-sdk` to generate Rust types corresponding to Move ones of their respective packages.

## Api documentation

It its recommended to check out the Rust documentation for the crates here. You can build it with
```
RUSTDOCFLAGS="-D warnings -Zunstable-options --generate-link-to-definition" cargo +nightly doc \
  -Zunstable-options \
  -Zrustdoc-map \
  --no-deps \
  --all-features
```

## Quickstart for interacting with Move packages

### Executing a transaction on Sui

The flow for creating and submitting a PTB usually goes like this:
```rust
use sui_crypto::Signer as _;
use sui_crypto::ed25519::Ed25519PrivateKey;
use sui_gql_client::reqwest::ReqwestClient;
use sui_jsonrpc::api::WriteApiClient as _;
use sui_jsonrpc::client::SuiClientBuilder;
use sui_jsonrpc::msgs::SuiTransactionBlockResponseOptions

let sender: Ed25519PrivateKey;
let sender_address = sender.public_key().to_address();

// 1. Instantiate RPC clients
let graphql = ReqwestClient::new_default("https://sui-mainnet.mystenlabs.com/graphql");
let jrpc = SuiClientBuilder::default().build("https://fullnode.mainnet.sui.io:443").await?;

// 2. Build the ProgrammableTransaction
let ptb = build_ptb(&graphql, &sender_address).await?;
let kind = af_sui_types::TransactionKind::ProgrammableTransaction(ptb);

// 3. Get the reference gas price if you don't know it already
let price = jrpc.http().get_reference_gas_price().await?.into_inner();
// 4. (Optionally) dry-run the PTB via the RPC to get an estimate of the gas budget necessary
let budget = jrpc.gas_budget(&kind, sender_address, price).await?;
// 5. Set a gas budget and query the RPC for gas coins with suffient total balance
let gas_data = jrpc.get_gas_data(&kind, sender_address, budget, price).await?;

// 6. Sign the transaction (we're using `sui_crypto` here)
let transaction = af_sui_types::TransactionData::v1(
    kind,
    sender_address,
    gas_data,
    af_sui_types::TransactionExpiration::None,
);
let signature: UserSignature = sender.sign(&transaction.signing_digest());

// 7. Serialize transaction and signatures then send then to the RPC
let resp = jrpc
    .http()
    .execute_transaction_block(
        transaction.encode_base64(),
        vec![signature.to_base64()],
        Some(SuiTransactionBlockResponseOptions::new().with_effects()),
        None,
    )
    .await?;
resp.check_execution_status()?;
```

Over time, a lot of JSON-RPC methods used above will be available through GraphQL, so that you can use only one client.

### Programmable Transaction Blocks (PTBs)

This defines what actually will be executed onchain. The recommended way to build programmable transactions is using the [`ptb!`](./crates/af-ptbuilder/src/sui/lib.rs) macro. 

```rust
use af_sui_types::ProgrammableTransaction;
use sui_gql_client::GraphQlClient;

/// In this example, we're interacting with a package `foo` which allows us to create and account
/// object holding coins of a specific type. We want to create an account for `SUI` coins and
/// transfer it back to the sender in one transaction.
async fn build_ptb(client: &impl GraphQlClient, sender: &Address) -> Result<ProgrammableTransaction>
{
    use af_ptbuilder::ptb;
    use sui_gql_client::object_args;

    // 1. Request information (`ObjectArg`) for the objects you're interacting with from the RPC
    object_args!({
        mut registry: "0x1e7f38ee60107485e03da942029146ceb283bba1f2db8b8ad305739f42b5ef36".parse()?,
        coin: "0x68c7d900be4bcb322342fd9bf53e06c537d92f5fa76ce5fb359703fa45beccdb".parse()?,
    } with { client });

    // For functions with type arguments, you need to explicitly pass them
    // For example, here we're constructing the one-time-witness type for SUI coins.
    let otw = "0x2::sui::SUI".parse()?;

    // 2. Build the programmable transaction containing the inputs (objects/values) and the sequence
    //    of Move calls operating on those inputs
    let ptb = ptb!(
        package foo: "<package-id>".parse()?;

        type T = otw;

        input obj registry;
        input obj coin;

        input pure sender;

        let account = foo::registry::create_account<T>(registry, coin);
        command! TransferObjects(vec![account], sender);
    );
    Ok(ptb)
}
```

Check out `ptb!`'s API documentation for the full syntax.


### Reading objects

There are helpers for fetching and parsing objects from Aftermath's packages on the Sui network. 

TODO: explain BCS deserialization and `sui_pkg_sdk!` macro.

The examples for [af-iperps](./crates/af-iperps/examples) make usage of the GraphQL APIs to fetch the protocol state (objects).
