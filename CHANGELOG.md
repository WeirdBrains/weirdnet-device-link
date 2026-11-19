# Changelog

All notable changes to weirdnet-device-link are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- Breaking (wire): builds on weirdnet-net 0.8.0, whose requests carry byte fields as CBOR byte strings.
- Breaking (wire): `/weirdnet/link/2` and `/weirdnet/link-invite/2` (were `/1`), so 0.1 and 0.2 devices fail to connect instead of failing to decode. Link both devices on the same version.

## [0.1.0] - 2026-11-16

### Added

- Device linking over a QR offer or invite, a 6-digit code check and a sealed identity transfer, moved from Morph.
- Builds on weirdnet-net 0.4.0.

[Unreleased]: https://github.com/WeirdBrains/weirdnet-device-link/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/WeirdBrains/weirdnet-device-link/compare/e106620...v0.1.0
