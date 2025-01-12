// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{StructTag, TypeTag, GAS_MODULE_NAME, GAS_STRUCT_NAME, SUI_FRAMEWORK_ADDRESS};

/// One-time-witness representation.
pub struct Gas;

impl Gas {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_FRAMEWORK_ADDRESS,
            name: GAS_STRUCT_NAME.to_owned(),
            module: GAS_MODULE_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(Self::type_()))
    }

    pub fn is_gas(other: &StructTag) -> bool {
        &Self::type_() == other
    }

    pub fn is_gas_type(other: &TypeTag) -> bool {
        match other {
            TypeTag::Struct(s) => Self::is_gas(s),
            _ => false,
        }
    }
}

/// Return `true` if `s` is the type of a gas coin (i.e., 0x2::coin::Coin<0x2::sui::SUI>)
pub fn is_gas_coin(s: &StructTag) -> bool {
    s.is_coin().is_some() && s.type_params.len() == 1 && Gas::is_gas_type(&s.type_params[0])
}
