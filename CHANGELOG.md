
* [a189de](https://github.com/terraswap/classic-terraswap/pull/15/commits/a189de88440f5a26e4104aa950fddbcb259a457e) Prevention of share calculation overflow
* [be08cb](https://github.com/terraswap/classic-terraswap/pull/15/commits/be08cb28d04b2310cadebe6a93a82d132303dc45) Refund & desired asset to provide liqudity
* [0fd802](https://github.com/terraswap/classic-terraswap/pull/15/commits/0fd802c4174c2d227b5aeeac79aa1558d3909493) Decimal loss is paid by the user
* [30aeb6](https://github.com/terraswap/classic-terraswap/pull/15/commits/30aeb63e7a4ee03d9c7e5930100d13e43556d761) min_assets support when withdraw
* [5b3ff3](https://github.com/terraswap/classic-terraswap/pull/15/commits/5b3ff3a768521cec114b9ea6be051adbbd5e6d8c) Support deadline
* [890a3b](https://github.com/terraswap/classic-terraswap/pull/15/commits/890a3b6414d8d78debe4d52bd1e583a9861acf9b) Revert slippage tolerance in provide liquidity
* [11b8e6](https://github.com/terraswap/classic-terraswap/pull/15/commits/11b8e68623d64835018e74a97c5a146d5abb3579) Initial liquidity deduction for pair protection
* [933ec4](https://github.com/terraswap/classic-terraswap/pull/15/commits/933ec4b53832227d1b0542c686ffa4977906ce05) Create pair with provide liquidity
* [57232e](https://github.com/terraswap/classic-terraswap/pull/15/commits/57232eb3c48f585ee53ba6286d96914b8c33d58a) cosmwasm v1.0

# 2.6.1

* [a7b30f](https://github.com/terraswap/classic-terraswap/pull/4/commits/a7b30f1c5e8585cdc05240dd8aca5f37a0765d96) Fixed parse decimals

# 2.6.0

* [f575f8](https://github.com/terraswap/classic-terraswap/pull/2/commits/f575f8e816fa551b4328f2e0acbbe9b001a71945) Tax on LUNC

# 2.5.2

* [319c39](https://github.com/terraswap/classic-terraswap/pull/3/commits/319c39b44889c2fc7a02add1ee99713a4a571124) rename classic_terraswap
* [db9848](https://github.com/terraswap/classic-terraswap/pull/1/commits/db98485e7c2b14a13c13a3b8fc0d68d07fed9dd1) Follow up latest Terraswap

# 2.5.1

* [2bfb0bb](https://github.com/terraswap/terraswap/pull/20/commits/82954c0aa289f12a3fe66df30cf1a65ce7bd4a4e) LOOP and ASTROPORT support on router

# 2.5.0

* [cd3cf2b](https://github.com/terraswap/terraswap/pull/30/commits/cd3cf2bb8d2438f5de4f5c1859b91fa46be85bf3) Support reverse simulate in router contract

# 2.4.1

* [191c1fb](https://github.com/terraswap/terraswap/pull/20/commits/191c1fb11e84771a022d793b70b9fe70988e50d3) Append `sender` and `receiver` event attributes to response.
* [f696e3b](https://github.com/terraswap/terraswap/pull/20/commits/f696e3b94d996ddf7fd10333519b82a904b834b1) Change github action's rust-optimizer to workspace-optimizer 

## Improvements 
InitHooks are removed from the contracts instantiate msg, instead it uses new reply feature of CosmWasm. 

# 2.4.0

Terraswap Initial Release
