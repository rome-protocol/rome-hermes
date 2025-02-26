use af_move_type::MoveType;
use af_utilities::{Balance9, IFixed};
use num_traits::Zero as _;

use crate::clearing_house::ClearingHouse;
use crate::market::MarketParams;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Overflow when converting types")]
    Overflow,
    #[error("Not enough precision to represent price")]
    Precision,
}

/// Convenience trait to convert to/from units used in the orderbook.
pub trait OrderBookUnits {
    fn price_to_ifixed(&self, price: u64) -> IFixed {
        let price_ifixed = IFixed::from(price);
        let lot_size_ifixed = IFixed::from(self.lot_size());
        let tick_size_ifixed = IFixed::from(self.tick_size());
        price_ifixed * tick_size_ifixed / lot_size_ifixed
    }

    /// The price in ticks/lot closest to the desired value.
    ///
    /// Note that this:
    /// - rounds the equivalent ticks/lot **down** to the nearest integer.
    /// - errors if the equivalent ticks/lot < 1, signaling not enough precision.
    fn ifixed_to_price(&self, ifixed: IFixed) -> Result<u64, Error> {
        if ifixed.is_zero() {
            return Ok(0);
        }
        // ifixed = (price_ifixed * tick_size_ifixed) / lot_size_ifixed
        // (ifixed * lot_size_ifixed) / tick_size_ifixed = price_ifixed
        let price_ifixed =
            (ifixed * IFixed::from(self.lot_size())) / IFixed::from(self.tick_size());
        let price: u64 = price_ifixed
            .integer()
            .uabs()
            .try_into()
            .map_err(|_| Error::Overflow)?;
        if price == 0 {
            return Err(Error::Precision);
        }
        Ok(price)
    }

    fn lots_to_ifixed(&self, lots: u64) -> IFixed {
        let ifixed_lots: IFixed = lots.into();
        let ifixed_lot_size: IFixed = Balance9::from_inner(self.lot_size()).into();
        ifixed_lots * ifixed_lot_size
    }

    fn ifixed_to_lots(&self, ifixed: IFixed) -> Result<u64, Error> {
        let balance: Balance9 = ifixed.try_into().map_err(|_| Error::Overflow)?;
        Ok(balance.into_inner() / self.lot_size())
    }

    // NOTE: these could be updated to return NonZeroU64 ensuring division by zero errors are
    // impossible.
    fn lot_size(&self) -> u64;
    fn tick_size(&self) -> u64;
}

impl OrderBookUnits for MarketParams {
    fn lot_size(&self) -> u64 {
        self.lot_size
    }

    fn tick_size(&self) -> u64 {
        self.tick_size
    }
}

impl<T: MoveType> OrderBookUnits for ClearingHouse<T> {
    fn lot_size(&self) -> u64 {
        self.market_params.lot_size
    }

    fn tick_size(&self) -> u64 {
        self.market_params.tick_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl OrderBookUnits for (u64, u64) {
        fn lot_size(&self) -> u64 {
            self.0
        }

        fn tick_size(&self) -> u64 {
            self.1
        }
    }

    #[test]
    fn orderbook_units() {
        let mut units = (10_000_000, 1_000_000);
        let mut ifixed: IFixed;

        ifixed = u64::MAX.into();
        ifixed += IFixed::from_inner(1.into());
        insta::assert_snapshot!(ifixed, @"18446744073709551615.000000000000000001");
        let err = units.ifixed_to_lots(ifixed).unwrap_err();
        insta::assert_snapshot!(err, @"Overflow when converting types");

        // Values smaller than 1 balance9 get cast to 0
        ifixed = IFixed::from_inner(1.into());
        insta::assert_snapshot!(ifixed, @"0.000000000000000001");
        let ok = units.ifixed_to_lots(ifixed).unwrap();
        assert_eq!(ok, 0);

        ifixed = 0.001.try_into().unwrap();
        insta::assert_snapshot!(ifixed, @"0.001");
        let err = units.ifixed_to_price(ifixed).unwrap_err();
        insta::assert_snapshot!(err, @"Not enough precision to represent price");

        ifixed = 0.0.try_into().unwrap();
        insta::assert_snapshot!(ifixed, @"0.0");
        let ok = units.ifixed_to_price(ifixed).unwrap();
        assert_eq!(ok, 0);

        ifixed = 0.1.try_into().unwrap();
        insta::assert_snapshot!(ifixed, @"0.1");
        let ok = units.ifixed_to_price(ifixed).unwrap();
        assert_eq!(ok, 1);

        // `ifixed_to_price` truncates
        ifixed = 0.15.try_into().unwrap();
        insta::assert_snapshot!(ifixed, @"0.15");
        let ok = units.ifixed_to_price(ifixed).unwrap();
        assert_eq!(ok, 1);

        ifixed = units.price_to_ifixed(0);
        insta::assert_snapshot!(ifixed, @"0.0");

        // Can handle an absurdly large price no problem
        units = (1, u64::MAX);
        let ok = units.price_to_ifixed(u64::MAX);
        insta::assert_snapshot!(ok, @"340282366920938463426481119284349108225.0");

        // Can handle an absurdly large lot size no problem
        units = (u64::MAX, 1);
        let ok = units.lots_to_ifixed(u64::MAX);
        insta::assert_snapshot!(ok, @"340282366920938463426481119284.349108225");
    }
}
