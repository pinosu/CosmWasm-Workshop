use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    Zero,
    Set(u8),
}

#[cw_serde]
pub enum ExecuteMsg {
    Inc,
    Dec,
    Set(u8),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CounterResponse)]
    Value,
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}
