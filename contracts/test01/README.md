# Test `Deps<TerraQuery>` with [cw-multi-test](https://crates.io/crates/cw-multi-test)

Trying to use [cw-multi-test](https://crates.io/crates/cw-multi-test) instantiate contract
when the contract entry points are in the format:

```text
instantiate(deps: DepsMut<TerraQuery>, env: Env, _info: MessageInfo, msg: InstantiateMsg)
```

where `TerraQuery` is from [`classic-bindings`](https://crates.io/crates/classic-bindings).
