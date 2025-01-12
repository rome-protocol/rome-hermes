// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::proc_macros::rpc;

#[rpc(client, namespace = "suix")]
pub trait DeepBookApi {
    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<String>;
}
