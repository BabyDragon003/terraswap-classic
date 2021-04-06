use cosmwasm_std::{ConversionOverflowError, OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not Reward or Order token")]
    UnacceptableToken {},

    #[error("Less CLSM amount than vesting amount")]
    LessThanVesting {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Max spread assertion")]
    MaxSpreadAssertion {},

    #[error("Asset mismatch")]
    AssetMismatch {},

    #[error("Min amount assertion ({min_asset} > {asset})")]
    MinAmountAssertion { min_asset: String, asset: String },

    #[error("Max slippage assertion")]
    MaxSlippageAssertion {},

    #[error("More initial liquidity needed ({min_lp_token} > {given_lp})")]
    MinimumLiquidityAmountError {
        min_lp_token: String,
        given_lp: String,
    },
}
