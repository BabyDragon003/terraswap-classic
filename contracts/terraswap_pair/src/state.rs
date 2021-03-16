use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use classic_terraswap::asset::PairInfoRaw;
use cw_storage_plus::Item;
use cosmwasm_std::{Addr};
    pub lunc_dynamic_mint: bool,
    pub ustc_dynamic_mint: bool,
    pub clsm_addr: Addr,
    pub moon_addr: Option<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAIR_INFO: Item<PairInfoRaw> = Item::new("pair_info");
