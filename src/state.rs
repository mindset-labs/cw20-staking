use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20Coin;
use cw_storage_plus::{Item, Map};

// Map<Staker Address, (Staked Balance, Last Reward Claim Block)>
pub const STAKED_BALANCES: Map<&Addr, (Cw20Coin, u64)> = Map::new("staked_balances");

// Reward per block
pub const REWARD_PER_BLOCK: Item<Uint128> = Item::new("reward_per_block");