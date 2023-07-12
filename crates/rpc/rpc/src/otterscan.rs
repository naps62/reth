use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use reth_primitives::{Address, BlockId};
use reth_rpc_api::{EthApiServer, OtterscanServer};

#[derive(Debug)]
pub struct OtterscanApi<Eth> {
    eth: Eth,
}

impl<Eth> OtterscanApi<Eth> {
    /// Creates a new instance of `Otterscan`.
    pub fn new(eth: Eth) -> Self {
        Self { eth }
    }
}

#[async_trait]
impl<Eth> OtterscanServer for OtterscanApi<Eth>
where
    Eth: EthApiServer,
{
    async fn has_code(&self, address: Address, block_number: Option<BlockId>) -> RpcResult<bool> {
        self.eth.get_code(address, block_number).await.map(|code| code.len() > 0)
    }
}
