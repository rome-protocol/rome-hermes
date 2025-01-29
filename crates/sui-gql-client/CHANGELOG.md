# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

## [0.14.5](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.14.4...sui-gql-client-v0.14.5)

### 📚 Documentation

- Regenerate changelogs from scratch - ([288008f](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/288008f5b60193ea34b765d8ad605cf4f25207e9))

## [0.14.4](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.14.3...sui-gql-client-v0.14.4)

### 🚜 Refactor

- Use `graphql-extract` as much as possible - ([e87241f](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/e87241f785d771d0b9a55d6ed54494e2a9a9cac4))

## [0.14.3](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.14.2...sui-gql-client-v0.14.3)

### 🚜 Refactor

- *(sui-gql-client)* Internal stream helpers - ([35e1d79](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/35e1d79193307d6d9be8068bb3b3d990d72f9277))
- Simplify type aliases and improve docs - ([f2ab829](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/f2ab829f3110ebc2fa9fb7ddbf91f0509e8b9a32))
- Inline all snapshot tests - ([c5fcff1](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/c5fcff103fe9e8667496359afadc2a71c3be9e0c))

## [0.14.1](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.14.0...sui-gql-client-v0.14.1)

### ⛰️ Features

- Add filtered_full_objects gql query - ([8640b5a](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/8640b5a9b5d47f79bb354d9eadb5f04632ef4298))

### 🐛 Bug Fixes

- Undo bump version - ([594b5f7](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/594b5f793e4a768cbba82d64c904063e8bb59718))
- Add new query to example in cargo.toml - ([41f8790](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/41f8790cd2199f7b329250e70d3bcce0da2ae0fb))
- Remove clippy unwrap_used - ([37319ff](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/37319ffb84cdf69609106becf72e886330895e08))

### 🚜 Refactor

- Change visibility of new ObjectFilter - ([a58a2aa](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/a58a2aa028ed9e22cb21d2fa3a192c57aefe2b11))
- Convert query to stream - ([3b92eb5](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/3b92eb526c2bdb0b50eb2b764e1d3c550af25dbe))
- Use String for type_ - ([5b81c1b](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/5b81c1b40eb5850efcb02e1cfd41032b360a2893))
- Make dependencies optional - ([177a6dd](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/177a6dd20ce2f625f2eb8c74208d2a37f4d23e68))
- Return Object only in the stream - ([031c8a9](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/031c8a97fdca13887076fad4e35032560eea9f78))
- Adapt example to new return type - ([55aaa7e](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/55aaa7e82c20f472667cda189850519000d32622))

### 📚 Documentation

- *(sui-gql-client)* Improvements - ([637fe1b](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/637fe1b6946f75d05cd7fb8bf1934d8e18b5d17f))

### ⚙️ Miscellaneous Tasks

- *(sui-gql-client)* Improve docs - ([fa74e4f](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/fa74e4fc935e5c545655e79b17d341bfd8e23e46))

## [0.14.0](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.13.4...sui-gql-client-v0.14.0)

### ⛰️ Features

- *(af-sui-types)* [**breaking**] Replace TransactionEffects - ([a22c555](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/a22c5558f9062c4a5111dfb1ff65ce98b9c169e1))
- *(sui-gql-client)* Deprecate `ObjectFilter::object_keys` - ([f43324f](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/f43324ff8175f8f9007672d73f39761b5ab770b4))
- *(sui-gql-client,sui-gql-schema)* [**breaking**] Remove `unstable` feature - ([d94a5e3](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/d94a5e3c610857f762c9e945dc1ed0cb31fd5edb))
- *(sui-gql-schema)* [**breaking**] Remove UInt53 scalar - ([4c503c7](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/4c503c72bae2686951f19fbb2e24474fb69fc4b0))

### ⚙️ Miscellaneous Tasks

- *(af-sui-types)* Bump incompat version [propagate] - ([fbf06ff](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/fbf06ff5b383d73297a7595b6a4ca7300bdbfbd2))
- *(af-sui-types)* [**breaking**] Bump to 0.7.0 - ([27e110a](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/27e110a9455d4a1b9c4d9c1a9e4e0c85728a1e96))
- Revert fbf06ff5 - ([8f2567b](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/8f2567b6efd2924092cb5a5a382a5cabeaf7fafd))

## [0.13.4](https://github.com/AftermathFinance/aftermath-sdk-rust/compare/sui-gql-client-v0.13.2...sui-gql-client-v0.13.4)

### ⛰️ Features

- *(crates)* Add remaining crates (#2) - ([5d2dae1](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/5d2dae1392de8ed6a5af63a0e559bd3416112b35))

### ⚙️ Miscellaneous Tasks

- Remove cyclical dev dependencies - ([08d9a17](https://github.com/AftermathFinance/aftermath-sdk-rust/commit/08d9a1710fb56c3a58663051eecf29a18e91594b))

<!-- generated by git-cliff -->
