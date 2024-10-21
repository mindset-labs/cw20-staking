use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::{
    AllAccountsResponse, AllAllowancesResponse, AllowanceResponse,
    BalanceResponse, Cw20ExecuteMsg, DownloadLogoResponse, MarketingInfoResponse, MinterResponse,
    TokenInfoResponse,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: Uint128,
    pub reward_per_block: Uint128,
    pub owner: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    // base cw20
    Cw20ExecuteMsg(Cw20ExecuteMsg),
    // staking
    Stake { amount: Uint128 },
    Unstake { amount: Uint128 },
    ClaimRewards { amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // // base cw20
    #[returns(BalanceResponse)]
    Balance { address: String },
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
    #[returns(MinterResponse)]
    Minter {},
    #[returns(MarketingInfoResponse)]
    MarketingInfo {},
    #[returns(DownloadLogoResponse)]
    DownloadLogo {},
    #[returns(AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    // staking
    #[returns(StakedBalanceResponse)]
    StakedBalance { address: Addr },
    #[returns(RewardResponse)]
    Reward { address: Addr },
}

#[cw_serde]
pub struct StakedBalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct RewardResponse {
    pub reward: Uint128,
}
