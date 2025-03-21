use eventsource_stream::Eventsource as _;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::Serialize;

use crate::{EncodingType, Error, PriceIdInput, PriceUpdate};

/// Streams
impl crate::PythClient {
    /// SSE route handler for streaming price updates.
    ///
    /// Arguments:
    /// * `ids`: Get the most recent price update for this set of price feed ids.
    /// * `encoding`: Optional encoding type. If set, return the price update in the encoding
    ///   specified by the encoding parameter. Default is [`EncodingType::Hex`].
    /// * `parsed`: If `true`, include the parsed price update in [`PriceUpdate::parsed`]. Defaults
    ///   to `false` for this client.
    /// * `allow_unordered`: If `true`, allows unordered price updates to be included in the stream.
    /// * `benchmarks_only`: If `true`, only include benchmark prices that are the initial price
    ///   updates at a given timestamp (i.e., prevPubTime != pubTime).
    ///
    /// /v2/updates/price/stream
    pub async fn stream_price_updates(
        &self,
        ids: Vec<PriceIdInput>,
        encoding: Option<EncodingType>,
        parsed: Option<bool>,
        allow_unordered: Option<bool>,
        benchmarks_only: Option<bool>,
    ) -> Result<impl Stream<Item = Result<PriceUpdate, Error>> + use<>, Error> {
        #[derive(Serialize)]
        struct Options {
            encoding: Option<EncodingType>,
            parsed: Option<bool>,
            allow_unordered: Option<bool>,
            benchmarks_only: Option<bool>,
        }

        let mut url = self.url.clone();
        url.set_path("/v2/updates/price/stream");

        let mut builder = self.client.get(url);
        for id in ids {
            builder = builder.query(&[("ids[]", id)]);
        }
        let request = builder
            .query(&Options {
                encoding,
                parsed: parsed.or(Some(false)),
                allow_unordered,
                benchmarks_only,
            })
            .build()
            .map_err(Error::RequestBuilder)?;

        let update_stream = self
            .client
            .execute(request)
            .await
            .map_err(Error::Execute)?
            .bytes_stream()
            .eventsource()
            .map_err(Error::EventStream)
            .map(|e| -> Result<PriceUpdate, _> {
                serde_json::from_str(&e?.data).map_err(Error::EventData)
            });
        Ok(update_stream)
    }
}
