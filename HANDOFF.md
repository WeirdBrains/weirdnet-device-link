# Handoff

Running notes for [weirdnet-device-link](https://github.com/WeirdBrains/weirdnet-device-link), newest first.

## 2026-11-22: main, Morph's generic device-link additions (unreleased)

- What changed: the same additions as on [maint/0.3](https://github.com/WeirdBrains/weirdnet-device-link/tree/maint/0.3), ported to main on top of 0.4.1: `offer_scheme`, `is_offer`, `known_relays`, `parse_relays`, `relays_from_env` and `link_node_config`. Additive only; dependency ranges unchanged; main stays publishable.
- Commits: [`01da492`](https://github.com/WeirdBrains/weirdnet-device-link/commit/01da4927b92bf4ac3072790863b9184e36cda88e) (a port of [`6bea8ee`](https://github.com/WeirdBrains/weirdnet-device-link/commit/6bea8ee2ad9ad8f6f4a161cfa9a156c9e0dade19)).
- Checks: cargo fmt --check, clippy --all-targets -D warnings, test (41 passed, 2 ignored), doc with -D warnings and package.
- Not tested: apps built against main.
- Left: not released; a 0.4.2 needs Michael's OK.
- Next: release 0.4.2 if approved.

## 2026-11-21: v0.4.1, public-content patch release

- What changed: v0.4.1 republishes the 0.4 line with the public-content rules applied (Weird Brains LLC as the copyright holder, m3talsmith as the author, no machine details), on top of `3f8aafa`. No code or API changes.
- Commits: `3f8aafa` and the `release: v0.4.1` commit tagged [v0.4.1](https://github.com/WeirdBrains/weirdnet-device-link/releases/tag/v0.4.1).
- Checks: cargo fmt, clippy, test and package; the packaged crate was scanned for private terms before publishing.
- Not tested: nothing beyond the crate's own test suite.
- Left: earlier versions on this line are yanked on [crates.io](https://crates.io/crates/weirdnet-device-link/versions).
- Next: nothing for this release.

