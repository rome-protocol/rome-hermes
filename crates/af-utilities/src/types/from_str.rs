// Copyright (c) 2023 The BigDecimal-rs Contributors
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

use af_sui_types::U256;

use super::IFixed;
use crate::I256;

const IFIXED_SCALE: i64 = 18;
const RADIX: u32 = 10;

#[derive(thiserror::Error, Debug)]
#[error("Parsing string '{string}': {error}")]
pub struct Error {
    pub string: String,
    pub error: String,
}

type Result<T> = std::result::Result<T, Error>;

pub(crate) fn ifixed_from_str(s: &str) -> Result<IFixed> {
    use std::str::FromStr as _;

    let exp_separator: &[_] = &['e', 'E'];

    // split slice into base and exponent parts
    let (base_part, exponent_value) = match s.find(exp_separator) {
        // exponent defaults to 0 if (e|E) not found
        None => (s, 0),

        // split and parse exponent field
        Some(loc) => {
            // slice up to `loc` and 1 after to skip the 'e' char
            let (base, e_exp) = s.split_at(loc);
            (
                base,
                i128::from_str(&e_exp[1..]).map_err(|e| Error {
                    string: s.to_owned(),
                    error: format!("Couldn't convert exponent to i128: {e:?}"),
                })?,
            )
        }
    };

    if base_part.is_empty() {
        return Err(Error {
            string: s.to_owned(),
            error: "Missing base part of the number".into(),
        });
    }

    let mut digit_buffer = String::new();

    let last_digit_loc = base_part.len() - 1;

    // split decimal into a digit string and decimal-point offset
    let (digits, decimal_offset) = match base_part.find('.') {
        // No dot! pass directly to BigInt
        None => (base_part, 0),
        // dot at last digit, pass all preceding digits to BigInt
        Some(loc) if loc == last_digit_loc => (&base_part[..last_digit_loc], 0),
        // decimal point found - necessary copy into new string buffer
        Some(loc) => {
            // split into leading and trailing digits
            let (lead, trail) = (&base_part[..loc], &base_part[loc + 1..]);

            digit_buffer.reserve(lead.len() + trail.len());
            // copy all leading characters into 'digits' string
            digit_buffer.push_str(lead);
            // copy all trailing characters after '.' into the digits string
            digit_buffer.push_str(trail);

            // count number of trailing digits
            let trail_digits = trail.chars().filter(|c| *c != '_').count();

            (digit_buffer.as_str(), trail_digits as i128)
        }
    };

    // Calculate scale by subtracing the parsed exponential
    // value from the number of decimal digits.
    let scale = decimal_offset
        .checked_sub(exponent_value)
        .and_then(|scale| i64::try_from(scale).ok())
        .ok_or_else(|| Error {
            string: s.to_owned(),
            error: format!("Exponent overflow when parsing '{}'", s),
        })?;

    let digits = if scale < IFIXED_SCALE {
        // If the scale is smaller than IFixed's, then we need more 0s for the underlying u256
        digits.to_owned()
            + &String::from_utf8(vec![b'0'; (IFIXED_SCALE - scale) as usize])
                .expect("0s are valid utf8")
    } else {
        // In this case, the number has more decimals than IFixed supports, so we truncate.
        digits[0..(digits.len() - (scale - IFIXED_SCALE) as usize)].to_owned()
    };
    dbg!(&digits);
    let is_neg = digits.starts_with('-');

    let u256_str = if is_neg { &digits[1..] } else { &digits };
    let inner = U256::from_str_radix(u256_str, RADIX).map_err(|e| Error {
        string: s.to_owned(),
        error: format!("Parsing inner u256: {e:?}"),
    })?;
    let unsigned = IFixed::from_inner(I256::from_inner(inner));
    Ok(if is_neg { -unsigned } else { unsigned })
}

#[cfg(test)]
fn ifixed_from_f64(v: f64) -> Result<IFixed> {
    ifixed_from_str(&v.to_string())
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::*;
    use crate::types::Fixed;

    impl IFixed {
        fn from_f64_faulty(value: f64) -> Self {
            let max_i256 = U256::max_value() >> 1;
            let unsigned_inner = Fixed::from(value.abs()).into_inner().min(max_i256);
            let unsigned_inner = I256::from_inner(unsigned_inner);
            Self::from_inner(if value.is_sign_negative() {
                -unsigned_inner
            } else {
                unsigned_inner
            })
        }
    }

    #[test]
    fn original_conversion() {
        let mut float = 0.001_f64;
        let ifixed = IFixed::from_f64_faulty(float);
        insta::assert_snapshot!(ifixed, @"0.001");

        float = 0.009;
        let ifixed = IFixed::from_f64_faulty(float);
        insta::assert_snapshot!(ifixed, @"0.008999999999999999");

        float = 0.003;
        let ifixed = IFixed::from_f64_faulty(float);
        insta::assert_snapshot!(ifixed, @"0.003");

        float = 1e-18;
        let ifixed = IFixed::from_f64_faulty(float);
        insta::assert_snapshot!(ifixed, @"0.000000000000000001");

        float = 2.2238;
        let ifixed = IFixed::from_f64_faulty(float);
        insta::assert_snapshot!(ifixed, @"2.223800000000000256");
    }

    #[test]
    fn new_conversion() {
        let mut float = 0.001_f64;
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.001");

        float = 0.009;
        insta::assert_snapshot!(float, @"0.009");

        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.009");

        float = 0.003;
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.003");

        float = 1e-18;
        insta::assert_snapshot!(float, @"0.000000000000000001");
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.000000000000000001");

        float = 2.2238;
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"2.2238");

        float = 2.3e+10;
        insta::assert_snapshot!(float, @"23000000000");
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"23000000000.0");

        float = 1.234567e+20;
        insta::assert_snapshot!(float, @"123456700000000000000");
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"123456700000000000000.0");

        let ifixed = ifixed_from_str("2.3e+10").unwrap();
        insta::assert_snapshot!(ifixed, @"23000000000.0");

        let ifixed = ifixed_from_str("1.234567e+20").unwrap();
        insta::assert_snapshot!(ifixed, @"123456700000000000000.0");

        float = -2.2238;
        let ifixed = ifixed_from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"-2.2238");

        let ifixed = ifixed_from_str("-1.234567e+20").unwrap();
        insta::assert_snapshot!(ifixed, @"-123456700000000000000.0");
    }
    //==============================================================================================
    // Previous attempt at a 'lossless' conversion
    //==============================================================================================

    #[test]
    fn conversion_via_bigdecimal() {
        let mut float = 0.001_f64;
        let ifixed = IFixed::from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.001");

        float = 0.009;
        insta::assert_snapshot!(float, @"0.009");

        let ifixed = IFixed::from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.009");

        float = 0.003;
        let ifixed = IFixed::from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.003");

        float = 1e-18;
        let ifixed = IFixed::from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"0.000000000000000001");

        float = 2.2238;
        let ifixed = IFixed::from_f64(float).unwrap();
        insta::assert_snapshot!(ifixed, @"2.2238");
    }

    /// Demonstrating why the documentation recommends against converting from f64 directly.
    ///
    /// > It is not recommended to convert a floating point number to a decimal directly, as the
    /// > floating point representation may be unexpected
    #[test]
    fn bigdecimal_native_conversion() {
        let float = 0.009;
        insta::assert_snapshot!(float, @"0.009");

        let bigd: BigDecimal = "0.009".parse().unwrap();
        insta::assert_snapshot!(bigd, @"0.009");

        let bigd: BigDecimal = float.try_into().unwrap();
        insta::assert_snapshot!(bigd, @"0.00899999999999999931998839741709161899052560329437255859375");

        let bigd: BigDecimal = float.to_string().parse().unwrap();
        insta::assert_snapshot!(bigd, @"0.009");

        let float = 2.2238;
        insta::assert_snapshot!(float, @"2.2238");

        let bigd: BigDecimal = "2.2238".parse().unwrap();
        insta::assert_snapshot!(bigd, @"2.2238");

        let bigd: BigDecimal = float.try_into().unwrap();
        insta::assert_snapshot!(bigd, @"2.223800000000000220978790821391157805919647216796875");

        let bigd: BigDecimal = float.to_string().parse().unwrap();
        insta::assert_snapshot!(bigd, @"2.2238");
    }

    type Result<T> = std::result::Result<T, Error>;

    #[derive(thiserror::Error, Debug)]
    #[non_exhaustive]
    enum Error {
        #[error("Couldn't convert from {value}: {error}")]
        FromF64 { value: f64, error: String },
    }

    trait FromF64: Sized {
        fn from_f64(value: f64) -> Result<Self>;
    }

    impl FromF64 for IFixed {
        fn from_f64(value: f64) -> Result<Self> {
            let decimal: BigDecimal = value.to_string().parse().map_err(|e| Error::FromF64 {
                value,
                error: format!("Parsing string representation: {e:?}"),
            })?;
            let bytes = decimal
                .with_scale(IFIXED_SCALE)
                .into_bigint_and_scale()
                .0
                .to_signed_bytes_le();
            let u256_bytes = get_bytes_padded(bytes).map_err(|vec| Error::FromF64 {
                value,
                error: format!("BigDecimal has too many bytes; {} > 32", vec.len()),
            })?;
            Ok(Self::from_inner(I256::from_inner(U256::from_le_bytes(
                &u256_bytes,
            ))))
        }
    }

    fn get_bytes_padded<const N: usize>(mut vec: Vec<u8>) -> std::result::Result<[u8; N], Vec<u8>> {
        if vec.len() > N {
            return Err(vec);
        }
        // Pad with zeros if len < N
        vec.resize(N, 0);
        Ok(vec.try_into().expect("len == N"))
    }
}
