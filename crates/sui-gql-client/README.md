<!-- cargo-rdme start -->

# Sui GraphQL client

First version of Aftermath's Sui GraphQL client using `cynic`.

The main item here is the `GraphQlClient` trait, defining the common
interface for clients interacting with an RPC. See the `reqwest` feature for a pre-made
implementation.

The queries inclued here (under feature `queries`) were constructed with the help of `cynic`s
[generator] and use the scalars defined in [`sui_gql_schema`].

## Custom queries

Users building their own queries should first:
1. add [`sui_gql_schema`] as a build dependency
1. register its schema in a `build.rs` file;
1. import the `schema` module in any module defining new fragments

For steps 1 and 2, you can check this crate's `[build-dependencies]` and `build.rs` for an
example of how to do so. Read more about schema crates in <https://cynic-rs.dev/large-apis>.

Then, to create query structs, we recommend using the [generator] with Sui's GraphQL
[schema][sui_schema] and to try reusing the scalars defined in `scalars`
as those automatically convert opaque types to more useful ones like [`af_sui_types`].

## Features

- `move-types`: compatibility with `af-move-type` types
- `mutations`: enables the `mutations` submodule
- `queries`: enables the `queries` submodule with pre-made queries
- `reqwest`: enables the `reqwest` submodule with an implementation of
  `GraphQlClient`
- `scalars`: re-exports the `scalars` module of [`sui_gql_schema`]

## Handy links:

- Query builder: [generator.cynic-rs.dev][generator]. When prompted either
  - click the "A URL" button and pass in:
    - `https://sui-testnet.mystenlabs.com/graphql` to build queries against the testnet schema
    - `https://sui-mainnet.mystenlabs.com/graphql` for the mainnet one
  - click the "I'll Paste It" button and paste the [schema][sui_schema]
- Cynic's [guide](https://cynic-rs.dev/)

[`sui_gql_schema`]: https://docs.rs/sui-gql-schema/latest/sui_gql_schema/
[generator]: https://generator.cynic-rs.dev/
[sui_schema]: https://github.com/MystenLabs/sui/blob/main/crates/sui-graphql-rpc/schema.graphql
[`af_sui_types`]: https://docs.rs/af-sui-types/latest/af_sui_types/

<!-- cargo-rdme end -->
