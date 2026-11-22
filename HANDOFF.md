# Handoff

Running notes for [weirdnet-device-link](https://github.com/WeirdBrains/weirdnet-device-link), newest first.

## 2026-11-22: maint/0.3, Morph's generic device-link additions (unreleased)

- What changed: the generic parts of Morph's device-link code moved here on the [maint/0.3](https://github.com/WeirdBrains/weirdnet-device-link/tree/maint/0.3) line, on top of 0.3.2: `offer_scheme` and `is_offer` (pair with several apps' codes) and `known_relays`, `parse_relays`, `relays_from_env` and `link_node_config` (link node relays and configuration). Additive only; dependency ranges unchanged.
- Commits: [`6bea8ee`](https://github.com/WeirdBrains/weirdnet-device-link/commit/6bea8ee2ad9ad8f6f4a161cfa9a156c9e0dade19), and [`bd52a86`](https://github.com/WeirdBrains/weirdnet-device-link/commit/bd52a8640b97d1ce8627d856a95cba2dc8e1b416) (the CHANGELOG's Unreleased link now compares from v0.3.2).
- Checks: cargo fmt --check, clippy --all-targets -D warnings, test (38 passed, 2 ignored) and doc with -D warnings.
- Not tested: Morph built against this branch (Morph pins 0.3.1 exactly).
- Left: the 0.3.3 release (needs Michael's OK on the exact question); the same additions as an unreleased commit on main.
- Next: release 0.3.3 once approved, update the [device-link guide](https://net.weirdbrains.com) on the site, then Morph moves to the crate's versions.

## 2026-11-21: v0.3.2, public-content patch release

- What changed: v0.3.2 republishes the 0.3 line with the public-content rules applied (Weird Brains LLC as the copyright holder, m3talsmith as the author, no machine details), on top of `89df54f`. No code or API changes. Released from the [maint/0.3](https://github.com/WeirdBrains/weirdnet-device-link/tree/maint/0.3) branch.
- Commits: `89df54f` and the `release: v0.3.2` commit tagged [v0.3.2](https://github.com/WeirdBrains/weirdnet-device-link/releases/tag/v0.3.2).
- Checks: cargo fmt, clippy, test and package; the packaged crate was scanned for private terms before publishing.
- Not tested: nothing beyond the crate's own test suite.
- Left: earlier versions on this line are yanked on [crates.io](https://crates.io/crates/weirdnet-device-link/versions).
- Next: nothing for this release.

