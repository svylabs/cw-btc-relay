use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
#[cw_serde]
pub struct InstantiateMsg {
    initial_header: Bytes32,
    initial_block_height: u32
}

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct Bytes32([u8; 32]);

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct Bytes4([u8; 4]);

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct Uint256(u128, u128);

#[cw_serde]
pub enum ExecuteMsg {
    AddHeader { header: Bytes32 }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BlockHeader)]
    GetBlockHeaderAtHeight { height: u32 },
    #[returns(bool)]
    ValidateTransactionHash { tx_hash: String, proof: String, height: u32 }
}

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub merkle_root: Bytes32,
    pub previous_header_hash: Bytes32,
    pub timestamp: u32,
    pub n_bits: u32,
    pub nonce: u32
}

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderCompact {
    pub merkle_root: Bytes32,
    pub previous_header_hash: Bytes32,
    pub compact_bytes: Bytes32
}

impl From<BlockHeader> for BlockHeaderCompact {
    fn from(value: BlockHeader) -> Self {
        todo!()
    }
}

impl From<Bytes32> for BlockHeader {
    fn from(value: Bytes32) -> Self {
        todo!()
    }
}

impl BlockHeader {
    pub fn hash() -> Bytes32 {
        // TODO: Calculate double sha256
        return Bytes32([0u8; 32])
    }
}
