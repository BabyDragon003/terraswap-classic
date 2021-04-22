use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use classic_terraswap::asset::PairInfo;
use classic_terraswap::factory::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, PairsResponse, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
