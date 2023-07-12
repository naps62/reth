use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth_primitives::{Address, BlockId};

/// Otterscan rpc interface.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "ots"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "ots"))]
pub trait Otterscan {
    /// Check if a certain address contains a deployed code.
    #[method(name = "hasCode")]
    async fn has_code(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<bool>;
}
