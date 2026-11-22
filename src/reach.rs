//! How a link node reaches its relays, and the limits it runs with.
//!
//! A relay usually has several addresses: TCP, QUIC and, where it serves
//! it, WebSocket over TLS on port 443 for networks that only let HTTPS out.
//! weirdnet-net (0.5 on) dials every configured address of a relay at once
//! and reserves a slot over the first that connects, and a request through
//! a circuit whose relay is not connected yet dials that relay over every
//! address it knows for it too. So a link node simply starts with its relays
//! in its [`NodeConfig`]. (Apps used to connect each relay themselves before
//! reserving, while weirdnet-net 0.4 dialed only the first address.)
//!
//! The circuits a node then listens on (and puts in its code) are the ones
//! the relay's reservation names, built from the addresses the relay
//! announces, whichever address carried the connection. A code only says
//! which relay to go through; the other device reaches that relay over
//! whatever address works on its own network.
use std::sync::Arc;
use std::time::Duration;

use weirdnet_net::addr::addr_peer;
use weirdnet_net::node::{Handler, Message, Node, NodeConfig};
use weirdnet_net::profile::Profile;
use weirdnet_net::{Keypair, Multiaddr, PeerId};

use crate::Result;

/// The largest request a link node takes. Link requests are a hello, a
/// sealed transfer (a 32-byte seed, a profile name, a node certificate and a
/// little settings JSON) or a cancel: a few KiB. Bulk data (identity sync)
/// belongs on a stream, not here. weirdnet-net's default is 8 MiB.
pub const REQUEST_MAX: u64 = 1024 * 1024;
/// The largest response: always `Ok` or a short refusal.
pub const RESPONSE_MAX: u64 = 16 * 1024;
/// How long a request waits for its answer. The other side answers at once
/// (the code check happens between requests, not inside one), so this only
/// has to cover a slow relayed connection. weirdnet-net's default is 60 s.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// `cfg` with the link limits wherever it leaves weirdnet-net's defaults.
pub fn with_link_limits(cfg: NodeConfig) -> NodeConfig {
    NodeConfig {
        request_max: if cfg.request_max == 0 { REQUEST_MAX } else { cfg.request_max },
        response_max: if cfg.response_max == 0 { RESPONSE_MAX } else { cfg.response_max },
        request_timeout: if cfg.request_timeout.is_zero() { REQUEST_TIMEOUT } else { cfg.request_timeout },
        ..cfg
    }
}

/// [`crate::KNOWN_RELAYS`] as multiaddrs: the relays a link node uses when
/// the app does not configure its own.
pub fn known_relays() -> Vec<Multiaddr> {
    crate::KNOWN_RELAYS.iter().filter_map(|a| a.parse().ok()).collect()
}

/// A comma-separated relay list (`/…/p2p/<relay>` multiaddrs) as multiaddrs,
/// in order. Entries that do not parse or name no relay peer id are dropped;
/// an empty list means no relays.
pub fn parse_relays(list: &str) -> Vec<Multiaddr> {
    list.split(',').filter_map(|a| a.trim().parse::<Multiaddr>().ok()).filter(|a| addr_peer(a).is_some()).collect()
}

/// The relays from the environment variable `var` ([`parse_relays`]; set
/// but empty means no relays), or [`known_relays`] when it is not set. Each
/// app picks its own variable name.
pub fn relays_from_env(var: &str) -> Vec<Multiaddr> {
    match std::env::var(var) {
        Ok(list) => parse_relays(&list),
        Err(_) => known_relays(),
    }
}

/// A link node's configuration: TCP and QUIC on every interface on a port
/// the system picks, `relays` (a circuit is reserved on each, so devices on
/// other networks connect), `label` for the node, and the link limits
/// ([`with_link_limits`]). Adjust the rest (discovery, extra listen
/// addresses) on the returned value.
pub fn link_node_config(label: &str, relays: Vec<Multiaddr>) -> NodeConfig {
    with_link_limits(NodeConfig { listen: NodeConfig::default_listen(0), relays, label: label.into(), ..Default::default() })
}

/// Relay addresses grouped by relay, both in configuration order. Addresses
/// without a `/p2p/` peer id are not relay addresses and are dropped, as are
/// repeats.
pub fn relay_groups(relays: &[Multiaddr]) -> Vec<(PeerId, Vec<Multiaddr>)> {
    let mut out: Vec<(PeerId, Vec<Multiaddr>)> = vec![];
    for a in relays {
        let Some(peer) = addr_peer(a) else { continue };
        match out.iter_mut().find(|(p, _)| *p == peer) {
            Some((_, addrs)) if addrs.contains(a) => {}
            Some((_, addrs)) => addrs.push(a.clone()),
            None => out.push((peer, vec![a.clone()])),
        }
    }
    out
}

/// A running link node and the relays it was configured with.
pub(crate) struct LinkNode<Q: Message, R: Message> {
    pub node: Arc<Node<Q, R>>,
    pub relays: Vec<(PeerId, Vec<Multiaddr>)>,
}

/// Starts a node with the link limits and the relays in `cfg` (weirdnet-net
/// reaches each over all of its addresses and reserves a slot in the
/// background: this returns once the node listens).
pub(crate) fn start<Q: Message, R: Message>(
    profile: &'static Profile,
    key: Keypair,
    cfg: NodeConfig,
    handler: Arc<dyn Handler<Q, R>>,
) -> Result<LinkNode<Q, R>> {
    let cfg = with_link_limits(cfg);
    let relays = relay_groups(&cfg.relays);
    let node = Arc::new(Node::start(profile, key, cfg, handler)?);
    Ok(LinkNode { node, relays })
}

#[cfg(test)]
mod tests {
    use super::*;

    const RELAY: &str = "12D3KooWHp56KezbVCubjuznJGqyn9xTe8PWePfuwv4YQUGxaRBB";

    fn m(a: &str) -> Multiaddr {
        a.parse().unwrap()
    }

    #[test]
    fn relays_group_by_peer_in_order_without_repeats() {
        let other = Keypair::generate_ed25519().public().to_peer_id();
        let addrs = [
            m(&format!("/dns4/relay.example/tcp/4747/p2p/{RELAY}")),
            m(&format!("/dns4/other.example/tcp/4747/p2p/{other}")),
            m(&format!("/dns4/relay.example/udp/4747/quic-v1/p2p/{RELAY}")),
            m(&format!("/dns4/relay.example/tcp/443/tls/ws/p2p/{RELAY}")),
            m(&format!("/dns4/relay.example/tcp/4747/p2p/{RELAY}")),
            m("/ip4/192.0.2.1/tcp/1"),
        ];
        let g = relay_groups(&addrs);
        assert_eq!(g.len(), 2);
        assert_eq!(g[0].0.to_string(), RELAY);
        assert_eq!(g[0].1, vec![addrs[0].clone(), addrs[2].clone(), addrs[3].clone()]);
        assert_eq!(g[1].1, vec![addrs[1].clone()]);
    }

    #[test]
    fn known_relays_all_parse_and_name_a_relay() {
        let r = known_relays();
        assert_eq!(r.len(), crate::KNOWN_RELAYS.len());
        assert!(r.iter().all(|a| addr_peer(a).is_some()));
        assert_eq!(relay_groups(&r).len(), 1);
    }

    #[test]
    fn a_relay_list_keeps_relay_addresses_in_order() {
        let list = format!(" /dns4/a.example/tcp/1/p2p/{RELAY} ,nonsense,/ip4/192.0.2.1/tcp/1, /dns4/b.example/udp/2/quic-v1/p2p/{RELAY}");
        assert_eq!(
            parse_relays(&list),
            vec![m(&format!("/dns4/a.example/tcp/1/p2p/{RELAY}")), m(&format!("/dns4/b.example/udp/2/quic-v1/p2p/{RELAY}"))]
        );
        assert!(parse_relays("").is_empty());
        assert!(parse_relays(" , ").is_empty());
    }

    #[test]
    fn relays_from_an_unset_variable_are_the_known_ones() {
        assert_eq!(relays_from_env("WEIRDNET_DEVICE_LINK_TEST_UNSET_RELAYS"), known_relays());
    }

    #[test]
    fn a_link_node_config_listens_everywhere_with_the_link_limits() {
        let c = link_node_config("app-link", known_relays());
        assert_eq!(c.listen, NodeConfig::default_listen(0));
        assert_eq!(c.relays, known_relays());
        assert_eq!(c.label, "app-link");
        assert_eq!((c.request_max, c.response_max, c.request_timeout), (REQUEST_MAX, RESPONSE_MAX, REQUEST_TIMEOUT));
    }

    #[test]
    fn link_limits_fill_only_what_is_left_at_the_default() {
        let c = with_link_limits(NodeConfig::default());
        assert_eq!((c.request_max, c.response_max, c.request_timeout), (REQUEST_MAX, RESPONSE_MAX, REQUEST_TIMEOUT));
        let c = with_link_limits(NodeConfig { request_max: 7, request_timeout: Duration::from_secs(3), ..Default::default() });
        assert_eq!((c.request_max, c.response_max, c.request_timeout), (7, RESPONSE_MAX, Duration::from_secs(3)));
    }
}
