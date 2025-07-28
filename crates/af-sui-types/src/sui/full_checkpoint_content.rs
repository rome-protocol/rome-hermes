// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sui_sdk_types::{Transaction, TransactionEvents};

use crate::sui::transaction::_serde::SignedTransactionWithIntentMessage;
use crate::{Object, SignedTransaction, TransactionEffects};

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointTransaction {
    /// The input Transaction
    #[serde_as(as = "SignedTransactionWithIntentMessage")]
    pub transaction: SignedTransaction,
    /// The effects produced by executing this transaction
    pub effects: TransactionEffects,
    /// The events, if any, emitted by this transaction during execution
    pub events: Option<TransactionEvents>,
    /// The state of all inputs to this transaction as they were prior to execution.
    pub input_objects: Vec<Object>,
    /// The state of all output objects created or mutated or unwrapped by this transaction.
    pub output_objects: Vec<Object>,
}

impl CheckpointTransaction {
    /// The unsigned transaction payload.
    pub const fn transaction_data(&self) -> &Transaction {
        &self.transaction.transaction
    }
}
