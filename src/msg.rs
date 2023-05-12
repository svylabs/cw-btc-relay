use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
#[cw_serde]
pub struct InstantiateMsg {
    initial_header: String,
    initial_block_height: u32
}

#[cw_serde]
pub enum ExecuteMsg {
    AddHeader { header: String }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BlockHeader)]
    GetBlockHeaderAtHeight { height: u32 },
    #[returns(bool)]
    ValidateTransactionHash { tx_hash: String, proof: String, height: u32 }
}

#[derive(Debug, Clone, JsonSchema, PartialEq)]
pub struct BlockHeader {
    version: u8,
    
}
