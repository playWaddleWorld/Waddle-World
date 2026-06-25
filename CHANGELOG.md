# Changelog

All notable changes to the Wheat.game protocol contracts are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- TypeScript test suite under `tests/` covering claim/abandon/upgrade/exchange/tribes/yield math.
- GitHub Actions CI: `anchor build`, `cargo fmt --check`, `anchor test`.
- `SECURITY.md` — vulnerability disclosure policy.
- `CONTRIBUTING.md` — contributor guidelines.

## [0.1.0] — 2026-05-04

### Added
- Initial Anchor program with `Plot`, `PlotOffer`, `Order`, `Tribe`, `Alliance` accounts (all PDAs).
- `claim_plot`, `abandon_plot`, `upgrade_plot` — staking lifecycle with USDC vault.
- `list_plot`, `accept_plot_offer` — P2P plot exchange with optimistic concurrency.
- `place_order`, `match_book` — continuous order book for crops, 2.5% protocol fee, self-match guard.
- `create_tribe`, `join_tribe`, `create_alliance`, `join_alliance` — social systems with member caps.
- `harvest` and `compute_yield` — multiplicative yield engine (tier × upgrade × tribe × alliance × Golden Hour).
- `is_golden_hour` — deterministic 6-hour cycle, 1-hour window, no oracle.
- 5-plot-per-wallet cap, enforced at program level.
- Abandon mechanics: 50% burn, 50% to protocol treasury.

[Unreleased]: https://github.com/leeoxiang/wheat-game/compare/v0.1.0...HEAD
[0.1.0]:      https://github.com/leeoxiang/wheat-game/releases/tag/v0.1.0
