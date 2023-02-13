use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub enum QueryMsg {
    ContractVersion {},
    Aggregate {
        queries: Vec<Call>,
    },
    TryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    TryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
    BlockAggregate {
        queries: Vec<Call>,
    },
    BlockTryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    BlockTryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
}

#[cw_serde]
pub struct Call {
    pub address: Addr,
    pub data: Binary,
}

#[cw_serde]
pub struct CallOptional {
    pub require_success: bool,
    pub address: Addr,
    pub data: Binary,
}

#[cw_serde]
#[derive(Default)]
pub struct CallResult {
    pub success: bool,
    pub data: Binary,
}

#[cw_serde]
pub struct AggregateResult {
    pub return_data: Vec<CallResult>,
}

#[cw_serde]
pub struct BlockAggregateResult {
    pub block: u64,
    pub return_data: Vec<CallResult>,
}

impl AggregateResult {
    pub fn from_return_data(return_data: Vec<CallResult>) -> AggregateResult {
        AggregateResult { return_data }
    }
}

impl BlockAggregateResult {
    pub fn from_return_data(block: u64, return_data: Vec<CallResult>) -> BlockAggregateResult {
        BlockAggregateResult { block, return_data }
    }
}
