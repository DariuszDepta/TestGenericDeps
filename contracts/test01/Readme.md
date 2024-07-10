# Test `Deps<TerraQuery>` with `cw-multitest`

trying to use cw-multitest instantiate contract when the contract entrypoints are in that format:
`instantiate( deps: DepsMut<TerraQuery>, env: Env, _info: MessageInfo, msg: InstantiateMsg,)`

`TerraQuery` is from `classic-bindings`
