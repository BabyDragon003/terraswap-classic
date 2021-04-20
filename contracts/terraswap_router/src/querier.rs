use cosmwasm_std::{Decimal, Decimal256, QuerierWrapper, StdResult, Uint128, Uint256};
use std::ops::Mul;

use classic_bindings::{TerraQuerier, TerraQuery};

static DECIMAL_FRACTION: Uint128 = Uint128::new(1_000_000_000_000_000_000u128);

pub fn compute_tax(
    querier: &QuerierWrapper<TerraQuery>,
    amount: Uint128,
    denom: String,
) -> StdResult<Uint128> {
    let terra_querier = TerraQuerier::new(querier);
    let tax_rate: Decimal = (terra_querier.query_tax_rate()?).rate;
    let tax_cap: Uint128 = (terra_querier.query_tax_cap(denom)?).cap;
    Ok(std::cmp::min(
        amount.checked_sub(amount.multiply_ratio(
            DECIMAL_FRACTION,
            DECIMAL_FRACTION * tax_rate + DECIMAL_FRACTION,
        ))?,
        tax_cap,
    ))
}

pub fn compute_reverse_tax(
    querier: &QuerierWrapper<TerraQuery>,
    amount: Uint128,
    denom: String,
) -> StdResult<Uint128> {
    let terra_querier = TerraQuerier::new(querier);
    let tax_rate: Decimal = (terra_querier.query_tax_rate()?).rate;
    let tax_cap: Uint128 = (terra_querier.query_tax_cap(denom)?).cap;

    let tax: Uint128 = (std::cmp::min(
        Uint256::from(amount).mul(Decimal256::one() + Decimal256::from(tax_rate)),
        Uint256::from(amount + tax_cap),
    ) - Uint256::from(amount))
    .try_into()?;

    Ok(tax)
}
