<!-- cargo-rdme start -->

Exports the `sui_pkg_sdk!` macro for generating Rust types from Move
source code and implementing relevant `af_move_type` traits.

Automates the conversion of Sui Move types to Rust. The goal is to extract as much information
as possible at compile time about the Move types of a Sui package, generating equivalent Rust
types that:
- are [BCS]-compatible with their on-chain counterparts, so that their contents can be
  deserialized from BCS bytes returned by RPCs
- embed type information based on their location (path) in a Move package + type parameters, so
  that a corresponding type tag can be easily constructed with just the missing information
- use the embedded type information when deserializing a `MoveInstance` to verify the type of
  incoming data, to avoid mistakenly deserializing a different type that has the same BCS bytes

See also:
- `af_move_type`
- [`af_move_type_derive`] for how the type tag information for a struct is derived from its
  declaration

<div class="warning">
This does not yet support Move 2024 fully.
</div>

# Move types to Rust types

This macro allows callers to *almost* copy and paste Move struct declarations and get equivalent
Rust types. Some additional steps may be necessary however:
- If `phantom` keywords are present, they must be substituted by `!phantom`
- Struct fields should be Rust types. That means they must be in scope. Common cases like
  `address`, `vector<T>` and `u256` are automatically handled.

You may use the second point above to leverage custom deserializations. For instance, if a
`u256` is supposed to be interpreted as a fixed point number, you may define a custom
`MoveType` that (de)serializes to/from `u256` but behaves like a fixed point number.

Additionally, you may add any outter attributes, e.g. docs, to structs and their fields.

All `MoveStruct`s created by this macro will have a pretty `Display`
using `tabled` as a backend.

[`af_move_type_derive`]: https://docs.rs/af-move-type-derive/latest/af_move_type_derive/derive.MoveStruct.html
[BCS]: https://docs.rs/bcs/latest/bcs/

## Examples
```rust
use af_sui_pkg_sdk::sui_pkg_sdk;

sui_pkg_sdk!(package {
    module clearing_house {
        /// Used to dynamically load market objects as needed.
        /// Used to dynamically load traders' position objects as needed.
        struct ClearingHouse<!phantom T> has key {
            id: UID,
            // ...
        }

        /// Stores all deposits from traders for collateral T.
        /// Stores the funds reserved for covering bad debt from untimely
        /// liquidations.
        ///
        /// The Clearing House keeps track of who owns each share of the vault.
        struct Vault<!phantom T> has key, store {
            id: UID,
            collateral_balance: Balance<T>,
            insurance_fund_balance: Balance<T>,
            scaling_factor: u64
        }
    }

    module keys {
        /// Key type for accessing trader position in clearing house.
        struct Position has copy, drop, store {
            account_id: u64,
        }
    }
});
```

Should generate:
```rust
pub mod clearing_house {
    use super::*;

    /// Used to dynamically load market objects as needed.
    /// Used to dynamically load traders' position objects as needed.
    #[derive(
        MoveStruct,
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        tabled::Tabled,
    )]
    #[derive(derive_new::new)]
    #[move_(module=clearing_house)]
    #[serde(bound(deserialize = ""))]
    #[allow(non_snake_case)]
    pub struct ClearingHouse<T: MoveType> {
        pub id: UID,
        #[serde(skip_deserializing, skip_serializing, default)]
        #[tabled(skip)]
        T: ::std::marker::PhantomData<T>,
    }

    impl<T: MoveType> std::fmt::Display for ClearingHouse<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let settings = af_sui_pkg_sdk::move_struct_table_option("ClearingHouse");
            let mut table = tabled::Table::new([self]);
            table.with(settings);
            f.write_fmt(format_args!("{0}", table))
        }
    }

    impl<T: MoveType> HasKey for ClearingHouse<T> {
        fn object_id(&self) -> ObjectId {
            self.id.id.bytes
        }
    }

    /// Stores all deposits from traders for collateral T.
    /// Stores the funds reserved for covering bad debt from untimely
    /// liquidations.
    ///
    /// The Clearing House keeps track of who owns each share of the vault.
    #[derive(
        MoveStruct,
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        tabled::Tabled,
    )]
    #[derive(derive_new::new)]
    #[move_(module=clearing_house)]
    #[serde(bound(deserialize = ""))]
    #[allow(non_snake_case)]
    pub struct Vault<T: MoveType> {
        pub id: UID,
        pub collateral_balance: Balance<T>,
        pub insurance_fund_balance: Balance<T>,
        pub scaling_factor: u64,
        #[serde(skip_deserializing, skip_serializing, default)]
        #[tabled(skip)]
        T: ::std::marker::PhantomData<T>,
    }

    impl<T: MoveType> std::fmt::Display for Vault<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let settings = af_sui_pkg_sdk::move_struct_table_option("Vault");
            let mut table = tabled::Table::new([self]);
            table.with(settings);
            f.write_fmt(format_args!("{0}", table))
        }
    }

    impl<T: MoveType> HasKey for Vault<T> {
        fn object_id(&self) -> ObjectId {
            self.id.id.bytes
        }
    }
}

pub mod keys {
    use super::*;

    /// Key type for accessing trader position in clearing house.
    #[derive(
        MoveStruct,
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        tabled::Tabled,
    )]
    #[derive(derive_new::new)]
    #[move_(module=keys)]
    #[serde(bound(deserialize = ""))]
    #[allow(non_snake_case)]
    pub struct Position {
        pub account_id: u64,
    }

    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let settings = af_sui_pkg_sdk::move_struct_table_option("Position");
            let mut table = tabled::Table::new([self]);
            table.with(settings);
            f.write_fmt(format_args!("{0}", table))
        }
    }
}
```

<!-- cargo-rdme end -->
