# Contributing to Croptopia

We're happy to have you. The protocol is small enough to read in an afternoon and rewards careful contributions.

## Quick start

```bash
# 1. Install Solana CLI + Anchor (https://www.anchor-lang.com/docs/installation)
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.30.1
avm use   0.30.1

# 2. Clone + install dependencies
git clone https://github.com/leeoxiang/wheat-game
cd wheat-game
yarn install

# 3. Build + run the test suite
anchor build
anchor test
```

## Before opening a PR

- [ ] `anchor build` passes with no warnings.
- [ ] `cargo fmt --check` passes.
- [ ] `anchor test` passes locally.
- [ ] New behavior is covered by a test under `tests/`.
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, etc.).

## What we'll merge

- Bug fixes with a failing test attached.
- Compute-unit optimizations with before/after numbers in the PR description.
- Documentation improvements.
- New features that fit the protocol scope — talk to us in an issue first if the change is non-trivial (>50 LoC or touches token flow).

## What we won't merge

- Changes that introduce off-chain dependencies in `programs/`.
- Drive-by reformatting unrelated to your patch.
- Features that require a non-Solana chain (the protocol is opinionated about its home).
- Anything that breaks the `MAX_PLOTS_PER_WALLET` invariant or the abandon-burn split without a written rationale.

## Coding style

- Rust 2021, Anchor `0.30.1`.
- Account discriminants are sacred — append fields, don't reorder.
- Custom `#[error_code]` errors only; no string-based panics.
- `emit!` events on every state mutation a UI would care about.
- Tier / multiplier values live as `const`, never as account fields.

## Security

If you've found a vulnerability, please follow [SECURITY.md](./SECURITY.md) and do **not** file a public issue.

---

Questions? Reach us at [@playCroptopia](https://x.com/playCroptopia) on X.
