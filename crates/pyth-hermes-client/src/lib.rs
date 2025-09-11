#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Client for [Pyth Hermes] using [`reqwest`]. See [`PythClient`](crate::PythClient).
//!
//! [Pyth Hermes]: https://docs.pyth.network/price-feeds/how-pyth-works/hermes
//! [`reqwest`]: https://docs.rs/reqwest/latest/reqwest/
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[cfg(feature = "stream")]
mod stream;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Building request payload: {0:?}")]
    RequestBuilder(reqwest::Error),

    #[error("Executing request to server: {0:?}")]
    Execute(reqwest::Error),

    #[error("Unsuccessful response status: {0:?}")]
    ResponseStatus(reqwest::Error),

    #[error("Deserializing response body: {0:?}")]
    Deserialize(reqwest::Error),

    #[cfg(feature = "stream")]
    #[error("From event stream: {0}")]
    EventStream(#[from] eventsource_stream::EventStreamError<reqwest::Error>),

    #[cfg(feature = "stream")]
    #[error("Deserializing event data: {0}")]
    EventData(serde_json::Error),
}

/// Client type for Pyth Hermes.
///
/// See the documentation for each endpoint in [Swagger](https://hermes.pyth.network/docs/).
#[derive(Debug, Clone)]
pub struct PythClient {
    client: reqwest::Client,
    url: url::Url,
}

impl PythClient {
    pub fn new(url: url::Url) -> Self {
        Self::new_with_client(Default::default(), url)
    }

    pub fn new_with_client(client: reqwest::Client, url: url::Url) -> Self {
        Self { client, url }
    }

    /// Build a client from [`PythClientConfig`].
    pub fn from_config(config: PythClientConfig) -> Result<Self, ConfigError> {
        let url = config.base_url_as_url()?;

        let mut headers = HeaderMap::new();
        if let Some(api_key) = config.api_key {
            let header_name = HeaderName::from_bytes(
                config
                    .api_key_header
                    .unwrap_or_else(|| "X-API-KEY".to_string())
                    .as_bytes(),
            )
            .map_err(|e| ConfigError::InvalidHeaderName(e.to_string()))?;

            let value_str = if header_name.as_str().eq_ignore_ascii_case("authorization")
                && !api_key
                    .trim_start()
                    .to_ascii_lowercase()
                    .starts_with("bearer ")
            {
                format!("Bearer {api_key}")
            } else {
                api_key
            };
            let header_value = HeaderValue::from_str(&value_str)
                .map_err(|e| ConfigError::InvalidHeaderValue(e.to_string()))?;
            headers.insert(header_name, header_value);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ConfigError::ClientBuild)?;

        Ok(Self::new_with_client(client, url))
    }

    /// Build a client from environment variables.
    ///
    /// - `PYTH_HERMES_BASE_URL` (optional): Base Hermes URL. Defaults to `https://hermes.pyth.network/`.
    /// - `PYTH_HERMES_API_KEY` (optional): If set, appended as a path segment to the base URL
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = PythClientConfig::from_env()?;
        Self::from_config(config)
    }

    fn endpoint(&self, path: &str) -> url::Url {
        let mut url = self.url.clone();
        let path_to_append = path.trim_start_matches('/');
        
        // If the base URL already has a path, append to it instead of replacing
        if !url.path().is_empty() && url.path() != "/" {
            let existing_path = url.path().trim_end_matches('/');
            url.set_path(&format!("{}/{}", existing_path, path_to_append));
        } else {
            url.set_path(&format!("/{}", path_to_append));
        }
        
        url
    }

    /// Get the set of price feeds.
    ///
    /// This endpoint fetches all price feeds from the Pyth network. It can be filtered by asset
    /// type and query string.
    ///
    /// Arguments:
    /// * `query`: If provided, the results will be filtered to all price feeds whose symbol
    ///   contains the query string. Query string is case insensitive. Example: "bitcoin"
    /// * `asset_type`: If provided, the results will be filtered by asset type.
    ///
    /// /v2/price_feeds
    pub async fn price_feeds(
        &self,
        query: String,
        asset_type: Option<AssetType>,
    ) -> Result<Vec<PriceFeedMetadata>, Error> {
        #[derive(Serialize)]
        struct Query {
            query: String,
            asset_type: Option<String>,
        }

        let url = self.endpoint("/v2/price_feeds");
        let request = self
            .client
            .get(url)
            .query(&Query {
                query,
                asset_type: asset_type.map(|a| a.to_string()),
            })
            .build()
            .map_err(Error::RequestBuilder)?;

        let result = self
            .client
            .execute(request)
            .await
            .map_err(Error::Execute)?
            .error_for_status()
            .map_err(Error::ResponseStatus)?
            .json()
            .await
            .map_err(Error::Deserialize)?;
        Ok(result)
    }

    /// Get the latest price updates by price feed id.
    ///
    /// Given a collection of price feed ids, retrieve the latest Pyth price for each price feed.
    ///
    /// Arguments:
    /// * `ids`: Get the most recent price update for this set of price feed ids.
    /// * `encoding`: Optional encoding type. If set, return the price update in the encoding
    ///   specified by the encoding parameter. Default is [`EncodingType::Hex`].
    /// * `parsed`: If `true`, include the parsed price update in [`PriceUpdate::parsed`]. Defaults
    ///   to `false` for this client.
    ///
    /// /v2/updates/price/latest
    pub async fn latest_price_update(
        &self,
        ids: Vec<PriceIdInput>,
        encoding: Option<EncodingType>,
        parsed: Option<bool>,
    ) -> Result<PriceUpdate, Error> {
        #[derive(Serialize)]
        struct Options {
            encoding: Option<EncodingType>,
            parsed: Option<bool>,
        }

        let url = self.endpoint("/v2/updates/price/latest");

        let mut builder = self.client.get(url);
        for id in ids {
            builder = builder.query(&[("ids[]", id)]);
        }
        let request = builder
            .query(&Options {
                encoding,
                parsed: parsed.or(Some(false)),
            })
            .build()
            .map_err(Error::RequestBuilder)?;

        let result = self
            .client
            .execute(request)
            .await
            .map_err(Error::Execute)?
            .error_for_status()
            .map_err(Error::ResponseStatus)?
            .json()
            .await
            .map_err(Error::Deserialize)?;
        Ok(result)
    }

    /// Get the latest price updates by price feed id.
    ///
    /// Given a collection of price feed ids, retrieve the latest Pyth price for each price feed.
    ///
    /// Arguments:
    /// * `publish_time`: The unix timestamp in seconds. This endpoint will return the first update
    ///   whose `publish_time` is >= the provided value.
    /// * `ids`: Get the price update for this set of price feed ids.
    /// * `encoding`: Optional encoding type. If set, return the price update in the encoding
    ///   specified by the encoding parameter. Default is [`EncodingType::Hex`].
    /// * `parsed`: If `true`, include the parsed price update in [`PriceUpdate::parsed`]. Defaults
    ///   to `false` for this client.
    ///
    /// /v2/updates/price/{publish_time}
    pub async fn price_update(
        &self,
        publish_time: u64,
        ids: Vec<PriceIdInput>,
        encoding: Option<EncodingType>,
        parsed: Option<bool>,
    ) -> Result<PriceUpdate, Error> {
        #[derive(Serialize)]
        struct Options {
            encoding: Option<EncodingType>,
            parsed: Option<bool>,
        }

        let url = self.endpoint(&format!("/v2/updates/price/{publish_time}"));

        let mut builder = self.client.get(url);
        for id in ids {
            builder = builder.query(&[("ids[]", id)]);
        }
        let request = builder
            .query(&Options {
                encoding,
                parsed: parsed.or(Some(false)),
            })
            .build()
            .map_err(Error::RequestBuilder)?;

        let result = self
            .client
            .execute(request)
            .await
            .map_err(Error::Execute)?
            .error_for_status()
            .map_err(Error::ResponseStatus)?
            .json()
            .await
            .map_err(Error::Deserialize)?;
        Ok(result)
    }
}

// =================================================================================================
//  Rust versions of the types in the Open API docs
// =================================================================================================

/// A price id is a 32-byte hex string, optionally prefixed with "0x".
///
/// Price ids are case insensitive.
///
/// See <https://pyth.network/developers/price-feed-ids> for a list of all price feed ids.
///
/// # Examples
///
/// * `0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43`
/// * `e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43`
pub type PriceIdInput = String;

/// Asset types for [`PythClient::price_feeds`].
#[derive(Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum AssetType {
    Crypto,
    Equity,
    Fx,
    Metal,
    Rates,
}

/// Entries in the array returned from [`PythClient::price_feeds`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PriceFeedMetadata {
    pub id: RpcPriceIdentifier,
    pub attributes: HashMap<String, String>,
}

/// Return type of [`PythClient::latest_price_update`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PriceUpdate {
    pub binary: BinaryPriceUpdate,
    pub parsed: Option<Vec<ParsedPriceUpdate>>,
}

/// Data to push onchain.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryPriceUpdate {
    pub data: Vec<String>,
    pub encoding: EncodingType,
}

impl BinaryPriceUpdate {
    /// Decoded price update.
    pub fn decode(&self) -> Result<Vec<Vec<u8>>, BinaryPriceUpdateError> {
        use base64::Engine as _;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let bytes_vec = match self.encoding {
            EncodingType::Hex => self
                .data
                .iter()
                .map(hex::decode)
                .collect::<Result<_, hex::FromHexError>>()?,
            EncodingType::Base64 => self
                .data
                .iter()
                .map(|d| BASE64.decode(d))
                .collect::<Result<_, base64::DecodeError>>()?,
        };
        Ok(bytes_vec)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum EncodingType {
    Hex,
    Base64,
}

/// Raw payload returned by the server.
///
/// Prefer converting this to a [`pyth_sdk::PriceFeed`] using [`TryInto`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParsedPriceUpdate {
    pub id: RpcPriceIdentifier,
    pub price: RpcPrice,
    pub ema_price: RpcPrice,
    pub metadata: RpcPriceFeedMetadataV2,
}

impl TryFrom<ParsedPriceUpdate> for pyth_sdk::PriceFeed {
    type Error = hex::FromHexError;

    fn try_from(value: ParsedPriceUpdate) -> Result<Self, Self::Error> {
        let ParsedPriceUpdate {
            id,
            price,
            ema_price,
            ..
        } = value;
        Ok(Self::new(
            pyth_sdk::PriceIdentifier::from_hex(id)?,
            price,
            ema_price,
        ))
    }
}

pub type RpcPriceIdentifier = String;

pub type RpcPrice = pyth_sdk::Price;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcPriceFeedMetadataV2 {
    pub prev_publish_time: Option<i64>,
    pub proof_available_time: Option<i64>,
    pub slot: Option<i64>,
}

/// For [`BinaryPriceUpdate::decode`].
#[derive(thiserror::Error, Debug)]
pub enum BinaryPriceUpdateError {
    #[error("Decoding hex payload: {0}")]
    HexDecode(#[from] hex::FromHexError),
    #[error("Decoding base64 payload: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}

// =================================================================================================
//  Configuration
// =================================================================================================

/// Configuration for constructing a [`PythClient`].
#[derive(Clone, Debug, Default)]
pub struct PythClientConfig {
    pub base_url: Option<String>,
    /// Optional API key to be sent as an HTTP header.
    pub api_key: Option<String>,
    /// Optional header name to carry the API key. Defaults to `X-API-KEY` if unset.
    pub api_key_header: Option<String>,
}

impl PythClientConfig {
    /// Load configuration from environment.
    ///
    /// - `PYTH_HERMES_BASE_URL` (optional)
    /// - `PYTH_HERMES_API_KEY` (optional)
    /// - `PYTH_HERMES_API_KEY_HEADER` (optional, defaults to `X-API-KEY`)
    pub fn from_env() -> Result<Self, ConfigError> {
        let base_url = std::env::var("PYTH_HERMES_BASE_URL").ok();
        let api_key = std::env::var("PYTH_HERMES_API_KEY").ok();
        let api_key_header = std::env::var("PYTH_HERMES_API_KEY_HEADER").ok();
        Ok(Self {
            base_url,
            api_key,
            api_key_header,
        })
    }

    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            base_url: Some(url.into()),
            api_key: None,
            api_key_header: None,
        }
    }

    /// Parse the base URL.
    fn base_url_as_url(&self) -> Result<url::Url, ConfigError> {
        let url = url::Url::parse(
            self.base_url
                .as_deref()
                .unwrap_or("https://hermes.pyth.network/"),
        )
        .map_err(ConfigError::InvalidUrl)?;
        Ok(url)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Invalid base URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Failed to build HTTP client: {0}")]
    ClientBuild(#[from] reqwest::Error),
    #[error("Invalid header name: {0}")]
    InvalidHeaderName(String),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(String),
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::LazyLock;

    use color_eyre::Result;
    use color_eyre::eyre::OptionExt as _;

    use super::*;

    static TEST_DATA: LazyLock<PathBuf> = LazyLock::new(|| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
    });

    #[test]
    fn price_update_deser() -> Result<()> {
        for file in std::fs::read_dir(TEST_DATA.join("latest_price"))? {
            let path = file?.path();
            let update: PriceUpdate = serde_json::from_slice(&std::fs::read(path)?)?;

            for parsed in update.parsed.ok_or_eyre("Missing parsed price update")? {
                let _: pyth_sdk::PriceFeed = parsed.try_into()?;
            }
        }
        Ok(())
    }
}
