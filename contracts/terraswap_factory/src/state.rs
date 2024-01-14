use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use classic_terraswap::asset::{AssetInfoRaw, AssetRaw, PairInfo, PairInfoRaw};
use cosmwasm_std::{Addr, Api, CanonicalAddr, Order, StdResult, Storage};
use cw_storage_plus::{Bound, Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pair_code_id: u64,
    pub token_code_id: u64,
    pub clsm_addr: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TmpPairInfo {
    pub pair_key: Vec<u8>,
    pub assets: [AssetRaw; 2],
    pub asset_decimals: [u8; 2],
    pub sender: Addr,
}

pub const TMP_PAIR_INFO: Item<TmpPairInfo> = Item::new("tmp_pair_info");
pub const PAIRS: Map<&[u8], PairInfoRaw> = Map::new("pair_info");

pub fn pair_key(asset_infos: &[AssetInfoRaw; 2]) -> Vec<u8> {
    let mut asset_infos = asset_infos.to_vec();
    asset_infos.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));

    [asset_infos[0].as_bytes(), asset_infos[1].as_bytes()].concat()
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;
pub fn read_pairs(
    storage: &dyn Storage,
    api: &dyn Api,
    start_after: Option<[AssetInfoRaw; 2]>,
    limit: Option<u32>,
) -> StdResult<Vec<PairInfo>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start(start_after).map(Bound::ExclusiveRaw);

    PAIRS
        .range(storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            v.to_normal(api)
        })
        .collect::<StdResult<Vec<PairInfo>>>()
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start(start_after: Option<[AssetInfoRaw; 2]>) -> Option<Vec<u8>> {
    start_after.map(|asset_infos| {
        let mut asset_infos = asset_infos.to_vec();
        asset_infos.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));

        let mut v = [asset_infos[0].as_bytes(), asset_infos[1].as_bytes()]
            .concat()
            .as_slice()
            .to_vec();
        v.push(1);
        v
    })
}

// key : asset info / value: decimals
pub const ALLOW_NATIVE_TOKENS: Map<&[u8], u8> = Map::new("allow_native_token");
pub fn add_allow_native_token(
    storage: &mut dyn Storage,
    denom: String,
    decimals: u8,
) -> StdResult<()> {
    ALLOW_NATIVE_TOKENS.save(storage, denom.as_bytes(), &decimals)
}

#[cfg(test)]
mod allow_native_token {

    use classic_terraswap::mock_querier::mock_dependencies;

    use super::*;

    #[test]
    fn normal() {
        let mut deps = mock_dependencies(&[]);
        let denom = "uluna".to_string();
        let decimals = 6u8;

        add_allow_native_token(deps.as_mut().storage, denom.to_string(), decimals).unwrap();

        assert_eq!(
            decimals,
            ALLOW_NATIVE_TOKENS
                .load(deps.as_ref().storage, denom.as_bytes())
                .unwrap()
        )
    }

    #[test]
    fn duplicate_register_will_append() {
        let mut deps = mock_dependencies(&[]);
        let denom = "uluna".to_string();

        add_allow_native_token(deps.as_mut().storage, denom.to_string(), 6u8).unwrap();

        assert_eq!(
            ALLOW_NATIVE_TOKENS
                .load(deps.as_ref().storage, denom.as_bytes())
                .unwrap(),
            6u8
        );

        add_allow_native_token(deps.as_mut().storage, denom.to_string(), 7u8).unwrap();
        assert_eq!(
            ALLOW_NATIVE_TOKENS
                .load(deps.as_ref().storage, denom.as_bytes())
                .unwrap(),
            7u8
        );
    }
}
