use af_utilities::IFixed;
use pyth_sdk::PriceIdentifier;

use crate::price_info::PriceInfoObject;

impl PriceInfoObject {
    /// Get the off chain price identifier from the Pyth sdk
    pub fn pyth_price_id(&self) -> PriceIdentifier {
        PriceIdentifier::new(
            self.price_info
                .price_feed
                .price_identifier
                .bytes
                .to_vec()
                .try_into()
                .expect("Validated lenght onchain"),
        )
    }

    pub fn get_pyth_price(&self) -> Result<IFixed, Error> {
        let pyth_price = &self.price_info.price_feed.price;
        let price = if pyth_price.price.negative {
            return Err(Error::NegativePythPrice(pyth_price.price.magnitude as i64));
        } else if pyth_price.expo.negative {
            IFixed::from(pyth_price.price.magnitude)
                / IFixed::from(u64::pow(10, pyth_price.expo.magnitude as u32))
        } else {
            IFixed::from(pyth_price.price.magnitude)
                * IFixed::from(u64::pow(10, pyth_price.expo.magnitude as u32))
        };

        Ok(price)
    }

    pub const fn get_timestamp_ms(&self) -> u64 {
        self.price_info.price_feed.price.timestamp * 1000
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Pyth price is negative: -{0}")]
    NegativePythPrice(i64),
}
