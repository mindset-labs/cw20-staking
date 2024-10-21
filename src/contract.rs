#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw20_base::msg::QueryMsg as Cw20BaseQueryMsg;
use cw20_base::contract::{instantiate as cw20_instantiate, execute as cw20_execute, query as cw20_query};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{REWARD_PER_BLOCK, STAKED_BALANCES};

const CONTRACT_NAME: &str = "crates.io:cw20-token-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    REWARD_PER_BLOCK.save(deps.storage, &msg.reward_per_block)?;
    cw20_instantiate(deps, env, info, cw20_base::msg::InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        initial_balances: vec![],
        mint: None,
        marketing: None,
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        ExecuteMsg::Cw20ExecuteMsg(cw20_msg) => cw20_execute(deps, env, info, cw20_msg)?,
        ExecuteMsg::Stake { amount } => unimplemented!(),
        ExecuteMsg::Unstake { amount } => unimplemented!(),
        ExecuteMsg::ClaimRewards { amount } => unimplemented!(),
    };

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::StakedBalance { address } => unimplemented!(),
        QueryMsg::Reward { address } => unimplemented!(),
        QueryMsg::Balance { address } => 
            cw20_query(deps, env, Cw20BaseQueryMsg::Balance { address }),
        QueryMsg::TokenInfo {} => 
            cw20_query(deps, env, Cw20BaseQueryMsg::TokenInfo {}),
        QueryMsg::Allowance { owner, spender } => 
            cw20_query(deps, env, Cw20BaseQueryMsg::Allowance { owner, spender }),
        QueryMsg::Minter {} =>  
            cw20_query(deps, env, Cw20BaseQueryMsg::Minter {}),
        QueryMsg::MarketingInfo {} => 
            cw20_query(deps, env, Cw20BaseQueryMsg::MarketingInfo {}),
        QueryMsg::DownloadLogo {} => 
            cw20_query(deps, env, Cw20BaseQueryMsg::DownloadLogo {}),
        QueryMsg::AllAllowances { owner, start_after, limit } => 
            cw20_query(deps, env, Cw20BaseQueryMsg::AllAllowances { owner, start_after, limit }),
        QueryMsg::AllAccounts { start_after, limit } => 
            cw20_query(deps, env, Cw20BaseQueryMsg::AllAccounts { start_after, limit }),
    }
}

#[cfg(test)]
mod tests {}
