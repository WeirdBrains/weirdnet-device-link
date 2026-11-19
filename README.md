# weirdnet-device-link

`weirdnet-device-link/v1`: move a Weirdnet user identity to another device of the same
person without typing its recovery words. Built in Morph ("Login over QR") and moved here
unchanged, now on weirdnet-net 0.8.0.

- **Offer**: the new device (no identity yet) shows a QR code or link with its node key, a
  fresh X25519 key, its addresses (a relay circuit too, so devices on other networks
  connect) and a 120-second expiry, signed by its node key. Nothing secret is in it.
- **Invite**: or the existing device shows the QR, and the new device scans it; the same
  offer, code check and transfer follow.
- **Session**: the existing device connects over `/weirdnet/link/2`; both derive a key and a
  6-digit short authentication string from the X25519 agreement and a transcript hash.
- **Transfer**: only after the user confirms the codes match on the existing device does it
  send the seed, the profile name and a node certificate for the new device, sealed with
  that key.
- An offer is single use and dies after three pairing attempts, a cancel, a transfer or its
  expiry.

Modules: `offer` (offers and links), `invite`, `session` (both sides without the network),
`net` (`HostNode`, `Pairing` over weirdnet-net), `wire` (CBOR messages).

## License

BSD-3-Clause.
