use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::{Asset, AssetInfo, VestInfo};

use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub clsm_addr: String,
    pub pair_vest: VestInfo,
    pub nft_vest: VestInfo,
    pub marketing_vest: VestInfo,
    pub game_vest: VestInfo,
    pub team_vest: VestInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MoonExecuteMsg {
    VestingMint {},
    DynamicMintFromLunc {
        amount: Uint128,
        price: Decimal
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

/// ReverseSimulationResponse returns reverse swap simulation response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ReverseSimulationResponse {
    pub offer_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
