#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw20::MinterResponse;
use cw20_base::msg::QueryMsg as Cw20BaseQueryMsg;
use cw20_base::contract::{
    instantiate as cw20_instantiate, query as cw20_query,
    execute_burn, execute_send, execute_transfer,
    execute_update_marketing, execute_upload_logo
};
use cw20_base::allowances::{
    execute_increase_allowance, execute_decrease_allowance,
    execute_transfer_from, execute_burn_from, execute_send_from,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::check_available_balance;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_available_balance, query_reward, query_staked_balance};
use crate::state::{StakingConfigs, STAKING_CONFIGS};

const CONTRACT_NAME: &str = "crates.io:cw20-token-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let minter = env.contract.address.to_string();
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STAKING_CONFIGS.save(deps.storage, &StakingConfigs { reward_per_block: msg.reward_per_block })?;
    cw20_instantiate(deps, env, info, cw20_base::msg::InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter,
            cap: None,
        }),
        marketing: msg.marketing,
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
        // handled by cw20_base
        ExecuteMsg::Transfer { recipient, amount } => {
            check_available_balance(&deps, &env, &info.sender, amount)?;
            execute_transfer(deps, env, info, recipient, amount)?
        },
        ExecuteMsg::Burn { amount } => {
            check_available_balance(&deps, &env, &info.sender, amount)?;
            execute_burn(deps, env, info, amount)?
        },
        ExecuteMsg::Send { contract, amount, msg } => {
            check_available_balance(&deps, &env, &info.sender, amount)?;
            execute_send(deps, env, info, contract, amount, msg)?
        },
        ExecuteMsg::IncreaseAllowance { spender, amount, expires } => 
            execute_increase_allowance(deps, env, info, spender, amount, expires)?,
        ExecuteMsg::DecreaseAllowance { spender, amount, expires } => 
            execute_decrease_allowance(deps, env, info, spender, amount, expires)?,
        ExecuteMsg::TransferFrom { owner, recipient, amount } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            check_available_balance(&deps, &env, &owner_addr, amount)?;
            execute_transfer_from(deps, env, info, owner, recipient, amount)?
        },
        ExecuteMsg::BurnFrom { owner, amount } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            check_available_balance(&deps, &env, &owner_addr, amount)?;
            execute_burn_from(deps, env, info, owner, amount)?
        },
        ExecuteMsg::SendFrom { owner, contract, amount, msg } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            check_available_balance(&deps, &env, &owner_addr, amount)?;
            execute_send_from(deps, env, info, owner, contract, amount, msg)?
        },
        ExecuteMsg::UpdateMarketing { project, description, marketing } => 
            execute_update_marketing(deps, env, info, project, description, marketing)?,
        ExecuteMsg::UploadLogo(logo) => 
            execute_upload_logo(deps, env, info, logo)?,
        
        // custom messages related to staking
        ExecuteMsg::Stake { amount } => unimplemented!(),
        ExecuteMsg::Unstake { amount } => unimplemented!(),
        ExecuteMsg::ClaimRewards { amount } => unimplemented!(),
        ExecuteMsg::Unlock {} => unimplemented!(),
    };

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::StakedBalance { address } => 
            to_json_binary(&query_staked_balance(deps, env, address)?),
        QueryMsg::Reward { address } => 
            to_json_binary(&query_reward(deps, env, address)?),
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
        QueryMsg::AvailableBalance { address } => 
            to_json_binary(&query_available_balance(deps, env, address)?),
    }
}

#[cfg(test)]
mod tests {}
