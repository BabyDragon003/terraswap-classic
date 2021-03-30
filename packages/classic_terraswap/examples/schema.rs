use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(PairInfo), &out_dir);
    export_schema(&schema_for!(Asset), &out_dir);
    export_schema(&schema_for!(AssetInfo), &out_dir);
}
