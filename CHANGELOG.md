# Changelog

All notable changes to weirdnet-device-link are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.3.0] - 2026-11-19

Releases as 0.3.0 (breaking: codes change).

### Added

- Compact codes: `compact` module with `KNOWN_RELAYS` and `pack_addr`/`unpack_addr`; a circuit through a known relay is one byte.
- Relay reach and status: `reach` (`with_link_limits`, `relay_groups`), `relays` (`RelayProbe`, `RelayState`, `relay_states`), `net_status()` on link nodes.
- `Pairing::send_with` and `session::make_transfer_with` carry sync opt-ins in the transfer.
- `Error::OlderCode` and `Error::NewerCode`, so apps can word version mismatches.
- `local-relay` example for headless checks.
- Fold in Morph's copy

### Changed

- Breaking (wire): offers and invites are compact CBOR arrays, protocol version 2. Older codes say "update" instead of "damaged".
- Offers carry up to three direct addresses (one per host first) and a circuit for each of up to two relays.
- Link nodes run with link-sized request and response limits and a 30 s timeout.

## [0.2.0] - 2026-11-19

### Changed

- Breaking (wire): builds on weirdnet-net 0.8.0, whose requests carry byte fields as CBOR byte strings.
- Breaking (wire): `/weirdnet/link/2` and `/weirdnet/link-invite/2` (were `/1`), so 0.1 and 0.2 devices fail to connect instead of failing to decode. Link both devices on the same version.

## [0.1.0] - 2026-11-16

### Added

- Device linking over a QR offer or invite, a 6-digit code check and a sealed identity transfer, moved from Morph.
- Builds on weirdnet-net 0.4.0.

[Unreleased]: https://github.com/WeirdBrains/weirdnet-device-link/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/WeirdBrains/weirdnet-device-link/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/WeirdBrains/weirdnet-device-link/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/WeirdBrains/weirdnet-device-link/compare/e106620...v0.1.0
