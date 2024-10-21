use cosmwasm_std::{Addr, Deps, Env, StdResult, Uint128};
use cw20_base::state::BALANCES;

use crate::state::{StakedBalance, LOCKED_BALANCES, STAKED_BALANCES, STAKING_CONFIGS};

pub fn query_staked_balance(deps: Deps, _env: Env, address: Addr) -> StdResult<Uint128> {
    let staked_balance = STAKED_BALANCES.may_load(deps.storage, &address)?.unwrap_or_default();
    Ok(staked_balance.amount)
}

pub fn query_reward(deps: Deps, env: Env, address: Addr) -> StdResult<Uint128> {
    let height = env.block.height;
    let staking_configs = STAKING_CONFIGS.load(deps.storage)?;
    let staked_balance = STAKED_BALANCES.may_load(deps.storage, &address)?;

    if let Some(StakedBalance { amount, last_reward_claim_block }) = staked_balance {
        let reward = staking_configs.reward_per_block * Uint128::from(height - last_reward_claim_block) * amount;
        Ok(reward)
    } else {
        Ok(Uint128::zero())
    }
}

pub fn query_available_balance(deps: Deps, _env: Env, address: Addr) -> StdResult<Uint128> {
    let total_balance = BALANCES.may_load(deps.storage, &address)?.unwrap_or_default();
    let staked_balance = STAKED_BALANCES.may_load(deps.storage, &address)?.unwrap_or_default();
    let locked_balance = LOCKED_BALANCES.may_load(deps.storage, &address)?.unwrap_or_default();

    Ok(total_balance - staked_balance.amount - locked_balance.amount)
}