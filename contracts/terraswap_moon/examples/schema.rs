use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use classic_terraswap::asset::MoonInfo;
use classic_terraswap::moon::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, PoolResponse, QueryMsg,
    ReverseSimulationResponse, SimulationResponse,
};
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(MoonInfo), &out_dir);
    export_schema(&schema_for!(PoolResponse), &out_dir);
    export_schema(&schema_for!(ReverseSimulationResponse), &out_dir);
    export_schema(&schema_for!(SimulationResponse), &out_dir);
}
