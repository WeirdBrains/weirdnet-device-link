# Security notes for weirdnet-device-link

Not audited.

- The offer carries no secret: a node key, an ephemeral X25519 public key, addresses and an
  expiry, signed by the node key. A changed offer fails its signature.
- The session key and the 6-digit code come from the X25519 agreement and a SHA-256
  transcript of the offer and the existing device's ephemeral key. Only comparing the codes
  on both screens rules out a man in the middle; the transfer waits for that confirmation.
- The transfer (seed, profile name, node certificate) is sealed with XChaCha20-Poly1305
  under the session key, bound to the transcript.
- An offer is single use: it dies after three pairing attempts, a cancel, a transfer or 120
  seconds. Secrets are zeroized and never logged.

Report security problems to m3talsmith@gmail.com.
