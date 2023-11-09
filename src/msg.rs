use cosmwasm_schema::{ cw_serde, QueryResponses };

#[cw_serde]
pub struct InstantiateMsg {
    pub fund: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddFunds {
        fund: i32,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetFund returns the current Fund as a json-encoded number
    #[returns(GetFundResponse)] GetFunds {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetFundResponse {
    pub fund: i32,
}
