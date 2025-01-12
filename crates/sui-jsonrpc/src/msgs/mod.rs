// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Originally `sui_json_rpc_types`. Renamed to stress that types here are used in JSON-RPC
//! communications.
#![allow(
    clippy::missing_const_for_fn,
    clippy::use_self,
    clippy::option_if_let_else
)]

pub use balance_changes::*;
pub use dynamic_field::*;
pub use object_changes::*;
use serde::{Deserialize, Serialize};
pub use sui_checkpoint::*;
pub use sui_coin::*;
pub use sui_event::*;
pub use sui_extended::*;
pub use sui_governance::*;
pub use sui_move::*;
pub use sui_object::*;
pub use sui_protocol::*;
pub use sui_transaction::*;

mod balance_changes;
mod displays;
mod dynamic_field;
mod object_changes;
mod sui_checkpoint;
mod sui_coin;
mod sui_event;
mod sui_extended;
mod sui_governance;
mod sui_move;
mod sui_object;
mod sui_protocol;
mod sui_transaction;

/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor` if
/// `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}

impl<T, C> Page<T, C> {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            next_cursor: None,
            has_next_page: false,
        }
    }
}
