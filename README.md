# weirdnet-device-link

`weirdnet-device-link/v1`: move a Weirdnet user identity to another device of the same
person without typing its recovery words. Built in Morph ("Login over QR") and moved here;
0.3 folds in the rest of Morph's copy (compact codes, relay reach and relay status). Builds
on weirdnet-net 0.8.0.

- **Offer**: the new device (no identity yet) shows a QR code or link with its node key, a
  fresh X25519 key, its addresses (up to three direct ones, one per host first, and a
  circuit for each of up to two relays, so devices on other networks connect) and a
  120-second expiry, signed by its node key. Nothing secret is in it. Codes are compact
  CBOR arrays (protocol version 2): addresses are binary multiaddrs, and a circuit through
  one of the `KNOWN_RELAYS` (the [WeirdBrains](https://weirdbrains.com) public relay) is a single byte, so the QR
  stays small.
- **Invite**: or the existing device shows the QR, and the new device scans it; the same
  offer, code check and transfer follow.
- **Session**: the existing device connects over `/weirdnet/link/2`; both derive a key and a
  6-digit short authentication string from the X25519 agreement and a transcript hash.
- **Transfer**: only after the user confirms the codes match on the existing device does it
  send the seed, the profile name, the sync types the user opted in to and a node
  certificate for the new device, sealed with that key (`Pairing::send_with`,
  `session::make_transfer_with`).
- **Service link** (0.4): for a server that must not hold the seed (a model or inference
  node), the existing device sends a `Credential` instead: the user's **public** key, a node
  certificate it signs with the user key (which stays on the existing device), the sync types
  and an opaque sync bundle (`weirdnet_sync::Identity::service_bundle`: the device's grant,
  the roster and, if it syncs types, epoch keys), sealed the same way
  (`Pairing::send_credential`, `session::make_credential`). The new device takes it with
  `take_any()` (`Received::Service`) and adopts it with `weirdnet_sync::Identity::adopt_service`.
  `take()` keeps returning only a full `Transfer`. A 0.3 device refuses a credential.
- An offer is single use and dies after three pairing attempts, a cancel, a transfer or its
  expiry.

- **Relays**: link nodes start with the relays in their `NodeConfig` and the link limits
  (`with_link_limits`: 1 MiB requests, 16 KiB responses, 30 s timeout, applied only where
  the config leaves weirdnet-net's defaults). `RelayProbe` is a short-lived node that
  reserves on the configured relays and measures each one's round trip with a names
  lookup; `relay_states` turns node statuses into per-relay connected, reserved and
  round-trip state for a settings page. `HostNode`, `Pairing` and `InviteHost` expose
  `net_status()`.

- **Several apps' codes**: `offer_scheme` tells which of several schemes a new device's
  code is on (the `<scheme>link/` prefix in any case) and normalizes it for `decode_link`,
  so an app can pair with another app's codes (the prefix is not signed). `is_offer` is the
  counterpart of `is_invite`.
- **Link node setup**: `link_node_config(label, relays)` listens on TCP and QUIC on every
  interface with the link limits; `known_relays()` is `KNOWN_RELAYS` as multiaddrs;
  `parse_relays` reads a comma-separated relay list and `relays_from_env(VAR)` reads one
  from an app's own environment variable (set but empty means no relays, unset means
  `known_relays()`).

Modules: `offer` (offers and links), `compact` (`KNOWN_RELAYS`, `pack_addr`/`unpack_addr`),
`invite`, `session` (both sides without the network), `net` (`HostNode`, `Pairing` over
weirdnet-net), `reach` (relay groups and link limits), `relays` (`RelayProbe`,
`relay_states`), `wire` (CBOR messages).

`cargo run --example local-relay` starts a relay and names server on this machine only, for
headless checks of linking and relay status.

## Moving to 0.3

- Codes changed (protocol `VERSION` 2: compact CBOR arrays). A 0.2 code read by 0.3, or
  the other way round, fails with "update" rather than "damaged". Link both devices on the
  same version. Stream ids stay `/weirdnet/link/2` and `/weirdnet/link-invite/2`.
- Version mismatches are now `Error::OlderCode` and `Error::NewerCode` (were
  `Error::Invalid`). Match them to word the message with your app's name.
- New: `compact`, `reach`, `relays`; `Pairing::send_with` (carries sync opt-ins and returns
  the new device's `NodeCert`); `session::make_transfer_with`; `net_status()` on
  `HostNode`, `Pairing` and `InviteHost`.
- Offers carry at most `MAX_DIRECT` (3) direct addresses plus a circuit for each of up to two relays (`MAX_CIRCUITS`).
- Link nodes now apply `with_link_limits` to the `NodeConfig` you pass.
- Moving from Morph's in-tree copy: depend on `weirdnet-device-link = "=0.3.0"`, delete
  `crates/weirdnet-device-link`, and move to `/weirdnet/link/2` (the in-tree copy used
  `/weirdnet/link/1`).

## License

BSD-3-Clause.
