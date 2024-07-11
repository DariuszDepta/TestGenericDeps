# Test generic dependencies

**cw-multi-test** instantiate contract with entry points
having specific `DepsMut<TerraQuery>` and `Deps<TerraQuery>` arguments, where
`TerraQuery` is from [`classic-bindings`](https://crates.io/crates/classic-bindings).

### Useful round-trip commands

Clean everything

```shell
cargo clean
```

Build the project

```shell
cargo build --workspace
```

Check the code with `clippy`

```shell
cargo clippy --all-targets --workspace
```

Run tests

```shell
cargo test --workspace
```
