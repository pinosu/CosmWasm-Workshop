# CosmWasm-Workshop

Getting Started with CosmWasm Smart Contracts

## Environment Setup

- Install Go 1.22: https://go.dev/doc/install
- Install Rust: https://www.rust-lang.org/tools/install
- Install Docker: https://docs.docker.com/engine/install/

- Add wasm target: 
`rustup target add wasm32-unknown-unknown`

- Install cargo-generate:
`cargo install cargo-generate`

## Counter Contract

### Generate contract from template

```sh
# Generate new contract from template
# Project Name: counter-contract
# minimal: true
cargo generate CosmWasm/cw-template
```

### Define messages

```rust
// File: src/msg.rs
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum InstantiateMsg {
    Zero {},
    Set {value: u8},
}

#[cw_serde]
pub enum ExecuteMsg {
    Inc {},
    Dec {},
    Set {value: u8},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CounterResponse)]
    Value {},
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}
```

### Implement contract entrypoints

```rust
// File: contract.rs
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, CounterResponse};
use cw_storage_plus::Item;

const COUNTER: Item<u8> = Item::new("value");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    COUNTER.save(
        deps.storage,
        &match msg {
            InstantiateMsg::Zero {} => 0,
            InstantiateMsg::Set {value} => value,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    COUNTER.update::<_, ContractError>(deps.storage, |old_value| {
        Ok(match msg {
            ExecuteMsg::Inc {} => old_value.saturating_add(1),
            ExecuteMsg::Dec {} => old_value.saturating_sub(1),
            ExecuteMsg::Set {value} => value,
        })
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value {} => Ok(to_json_binary(&CounterResponse {
            value: COUNTER.may_load(deps.storage)?.unwrap(),
        })?),
    }
}
```

## Build contract

For local testing:
```sh
cargo build --target wasm32-unknown-unknown --release --lib
# The artifact will be generated in the ./target/wasm32-unknown-unknown/release/ directory.
```

For testnet/mainnet use the CosmWasm optimizer:
```sh
docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/optimizer:0.16.0

# The artifact will be generated in the ./artifacts directory.
```

## Neutron testnet

### Setup client

1. Clone the repo from here: https://github.com/neutron-org/neutron
2. Checkout the right version: `git checkout v5.0.2`
3. cd to it and run `make install`
4. make sure neutrond binary is available by running `neutrond version`

### Configure default values for neutron testnet
1. `neutrond config chain-id pion-1`
2. `neutrond config keyring-backend test`
3. `neutrond config node https://rpc-palvus.pion-1.ntrn.tech:443`

### Create an account
1. Add a new key: `neutrond keys add cw-wallet`
2. visit https://discord.neutron.org
3. find `testnet-faucet` channel in the `GENERAL` section
4. type: `$request <addr>`

### Upload contract to the network
```sh
cd artifacts
neutrond  tx wasm store "./counter_contract.wasm"  --from cw-wallet --gas 2500000 --gas-prices 0.025untrn -y
```

### Instantiate the contract
```sh
neutrond tx wasm instantiate <code_id> '{"zero":{}}' --from cw-wallet --admin cw-wallet --label counter --gas 250000 --gas-prices 0.025untrn -y
```

### Query contract state
All
```sh
neutrond q wasm contract-state all <contract_address>
```

Raw
```sh
neutrond q wasm contract-state raw <contract_address> <key>
```

Smart
```sh
neutrond q wasm contract-state smart <contract_address> '{"value":{}}'
```

### Execute msg

Increment counter by 1
```sh
neutrond tx wasm execute <contract_address> '{"inc":{}}' --from cw-wallet --gas 250000 --gas-prices 0.025untrn -y
```

Decrement counter by 1
```sh
neutrond tx wasm execute <contract_address> '{"dec":{}}' --from cw-wallet --gas 250000 --gas-prices 0.025untrn -y
```

Set counter to 10
```sh
neutrond tx wasm execute <contract_address> '{"set":{"value": 10}}' --from cw-wallet --gas 250000 --gas-prices 0.025untrn -y
```

## References:

- Cosmwasm documentation: https://docs.cosmwasm.com/
- Neutron Nebular workshop: https://github.com/sotnikov-s/nnw-helper/