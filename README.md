# Classic TerraSwap
[![classic_terraswap on crates.io](https://img.shields.io/crates/v/classic_terraswap.svg)](https://crates.io/crates/classic_terraswap)
[![workflow](https://github.com/terraswap/classic-terraswap/actions/workflows/tests.yml/badge.svg)](https://github.com/terraswap/classic-terraswap/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/terraswap/classic-terraswap/branch/main/graph/badge.svg?token=ERMFLEY6Y7)](https://codecov.io/gh/terraswap/classic-terraswap)

Uniswap-inspired automated market-maker (AMM) protocol powered by Smart Contracts on the [Terra](https://terra.money) blockchain.

## Contracts

| Name                                               | Description                                  |

   Mainnet: `terra1g3zc8lwwmkrm0cz9wkgl849pdqaw6cq8lh7872`

## Running this contract

You will need Rust 1.44.1+ with wasm32-unknown-unknown target installed.

You can run unit tests on this on each contracts' directory via :

```
cargo unit-test
cargo integration-test
```

Once you are happy with the content, you can compile it to wasm on each contracts directory via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

Or for a production-ready (compressed) build, run the following from the repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

The optimized contracts are generated in the artifacts/ directory.
