# Contributing to sentinel-contract

## Setup
```
cargo build --target wasm32-unknown-unknown --release
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Before opening a PR
- Run the commands above and make sure they pass.
- Keep the PR scoped to one issue; reference it with `Closes #N`.
- Update README.md if you changed public behavior.

## Related repos
- [specula-api](https://github.com/Specula-Labs/specula-api) — reads contract events and exposes them through its API; it does not currently submit transactions.
- [specula-app](https://github.com/Specula-Labs/specula-app) — dashboard displaying screening results and backend-provided contract events.
- [specula-api](https://github.com/Specula-Labs/specula-api) — the documented public backend repository for this contract.
