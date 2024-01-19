//! # P2P
//!
//! The underlying technology for managing the peer-to-peer networking used by this package is the [`rust-libp2p`](https://github.com/libp2p/rust-libp2p) library ([documentation](https://docs.libp2p.io/)).
//!
//! ## Modularity
//!
//! `rust-libp2p` is highly modular allowing for reimplmenting expected behavior using custom implementations for API traits.
//!
//! This way it is possible to experiment with and combine different components of the library in order to construct a specific targeted use case.
//!
//! ## `rust-libp2p` connectivity
//!
//! As per the [official documentation](https://connectivity.libp2p.io/), the connectivity types in the library are divided into the `standalone` (implementation of network over host) and `browser` (implementation of network over browser).
//!
//! Nodes that are not located behind a blocking firewall or NAT are designated as **public nodes** and can utilize the `TCP` or `QUIC` connectivity, with the recommendation to use QUIC if possible.
//!
//! Browser based solutions are almost always located behind a private network or a blocking firewall and to open a connection towards the standalone nodes these utilize either the `WebSocket` approach (by hijacking the `TCP` connection) or the (not yet fully speced up) `WebTransport` (by hijacking the `QUIC` connection).
//!

pub mod api;
pub mod errors;

use std::fmt::Debug;

use core_protocol::{
    ack::config::AckProtocolConfig,
    config::ProtocolConfig,
    constants::{
        self, HOPR_ACKNOWLEDGE_PROTOCOL_V_0_1_0, HOPR_HEARTBEAT_PROTOCOL_V_0_1_0, HOPR_MESSAGE_PROTOCOL_V_0_1_0,
        HOPR_TICKET_AGGREGATION_PROTOCOL_V_0_1_0,
    },
    heartbeat::config::HeartbeatProtocolConfig,
    msg::config::MsgProtocolConfig,
    ticket_aggregation::config::TicketAggregationProtocolConfig,
};

pub use libp2p;

use libp2p::{swarm::NetworkBehaviour, StreamProtocol};

use serde::{Deserialize, Serialize};

use core_network::messaging::ControlMessage;
use hopr_internal_types::acknowledgement::Acknowledgement;
use hopr_internal_types::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ping(pub ControlMessage);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pong(pub ControlMessage, pub String);

/// Network Behavior definition for aggregated HOPR network functionality.
///
/// Individual network behaviors from the libp2p perspectives are aggregated
/// under this type in order to create an aggregated network behavior capable
/// of generating events for all component behaviors.
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "HoprNetworkBehaviorEvent")]
pub struct HoprNetworkBehavior {
    pub heartbeat: libp2p::request_response::cbor::Behaviour<Ping, Pong>,
    pub msg: libp2p::request_response::cbor::Behaviour<Box<[u8]>, ()>,
    pub ack: libp2p::request_response::cbor::Behaviour<Acknowledgement, ()>,
    pub ticket_aggregation:
        libp2p::request_response::cbor::Behaviour<Vec<AcknowledgedTicket>, std::result::Result<Ticket, String>>,
}

impl Debug for HoprNetworkBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoprNetworkBehavior").finish()
    }
}

impl HoprNetworkBehavior {
    pub fn new(
        msg_cfg: MsgProtocolConfig,
        ack_cfg: AckProtocolConfig,
        hb_cfg: HeartbeatProtocolConfig,
        ticket_aggregation_cfg: TicketAggregationProtocolConfig,
    ) -> Self {
        Self {
            heartbeat: libp2p::request_response::cbor::Behaviour::<Ping, Pong>::new(
                [(
                    StreamProtocol::new(HOPR_HEARTBEAT_PROTOCOL_V_0_1_0),
                    libp2p::request_response::ProtocolSupport::Full,
                )],
                libp2p::request_response::Config::default().with_request_timeout(hb_cfg.timeout),
            ),
            msg: libp2p::request_response::cbor::Behaviour::<Box<[u8]>, ()>::new(
                [(
                    StreamProtocol::new(HOPR_MESSAGE_PROTOCOL_V_0_1_0),
                    libp2p::request_response::ProtocolSupport::Full,
                )],
                libp2p::request_response::Config::default().with_request_timeout(msg_cfg.timeout),
            ),
            ack: libp2p::request_response::cbor::Behaviour::<Acknowledgement, ()>::new(
                [(
                    StreamProtocol::new(HOPR_ACKNOWLEDGE_PROTOCOL_V_0_1_0),
                    libp2p::request_response::ProtocolSupport::Full,
                )],
                libp2p::request_response::Config::default().with_request_timeout(ack_cfg.timeout),
            ),
            ticket_aggregation: libp2p::request_response::cbor::Behaviour::<
                Vec<AcknowledgedTicket>,
                std::result::Result<Ticket, String>,
            >::new(
                [(
                    StreamProtocol::new(HOPR_TICKET_AGGREGATION_PROTOCOL_V_0_1_0),
                    libp2p::request_response::ProtocolSupport::Full,
                )],
                libp2p::request_response::Config::default().with_request_timeout(ticket_aggregation_cfg.timeout),
            ),
        }
    }
}

impl Default for HoprNetworkBehavior {
    fn default() -> Self {
        Self::new(
            MsgProtocolConfig::default(),
            AckProtocolConfig::default(),
            HeartbeatProtocolConfig::default(),
            TicketAggregationProtocolConfig::default(),
        )
    }
}

/// Aggregated network behavior event inheriting the component behaviors' events.
///
/// Necessary to allow the libp2p handler to properly distribute the events for
/// processing in the business logic loop.
#[derive(Debug)]
pub enum HoprNetworkBehaviorEvent {
    Heartbeat(libp2p::request_response::Event<Ping, Pong>),
    Message(libp2p::request_response::Event<Box<[u8]>, ()>),
    Acknowledgement(libp2p::request_response::Event<Acknowledgement, ()>),
    TicketAggregation(libp2p::request_response::Event<Vec<AcknowledgedTicket>, std::result::Result<Ticket, String>>),
    KeepAlive(void::Void),
}

impl From<void::Void> for HoprNetworkBehaviorEvent {
    fn from(event: void::Void) -> Self {
        Self::KeepAlive(event)
    }
}

impl From<libp2p::request_response::Event<Ping, Pong>> for HoprNetworkBehaviorEvent {
    fn from(event: libp2p::request_response::Event<Ping, Pong>) -> Self {
        Self::Heartbeat(event)
    }
}

impl From<libp2p::request_response::Event<Box<[u8]>, ()>> for HoprNetworkBehaviorEvent {
    fn from(event: libp2p::request_response::Event<Box<[u8]>, ()>) -> Self {
        Self::Message(event)
    }
}

impl From<libp2p::request_response::Event<Vec<AcknowledgedTicket>, std::result::Result<Ticket, String>>>
    for HoprNetworkBehaviorEvent
{
    fn from(
        event: libp2p::request_response::Event<Vec<AcknowledgedTicket>, std::result::Result<Ticket, String>>,
    ) -> Self {
        Self::TicketAggregation(event)
    }
}

impl From<libp2p::request_response::Event<Acknowledgement, ()>> for HoprNetworkBehaviorEvent {
    fn from(event: libp2p::request_response::Event<Acknowledgement, ()>) -> Self {
        Self::Acknowledgement(event)
    }
}

/// Build objects comprising the p2p network.
///
/// @return A built `Swarm` object implementing the HoprNetworkBehavior functionality
pub async fn build_p2p_network(
    me: libp2p::identity::Keypair,
    protocol_cfg: ProtocolConfig,
) -> crate::errors::Result<libp2p::Swarm<HoprNetworkBehavior>> {
    Ok(libp2p::SwarmBuilder::with_existing_identity(me)
        .with_async_std()
        .with_tcp(Default::default(), libp2p::noise::Config::new, || {
            let mut mplex_config = libp2p_mplex::MplexConfig::new();

            // libp2p default is 128
            // we use more to accomodate many concurrent messages
            // FIXME: make value configurable
            mplex_config.set_max_num_streams(1024);

            // libp2p default is 32 Bytes
            // we use the default for now
            // FIXME: benchmark and find appropriate values
            mplex_config.set_max_buffer_size(32);

            // libp2p default is 8 KBytes
            // we use the default for now, max allowed would be 1MB
            // FIXME: benchmark and find appropriate values
            mplex_config.set_split_send_size(8 * 1024);

            // libp2p default is Block
            // Alternative is ResetStream
            // FIXME: benchmark and find appropriate values
            mplex_config.set_max_buffer_behaviour(libp2p_mplex::MaxBufferBehaviour::Block);

            mplex_config
        })
        .map_err(|e| crate::errors::P2PError::Libp2p(e.to_string()))?
        .with_dns()
        .await
        .map_err(|e| crate::errors::P2PError::Libp2p(e.to_string()))?
        .with_behaviour(|_key| {
            HoprNetworkBehavior::new(
                protocol_cfg.msg,
                protocol_cfg.ack,
                protocol_cfg.heartbeat,
                protocol_cfg.ticket_aggregation,
            )
        })
        .map_err(|e| crate::errors::P2PError::Libp2p(e.to_string()))?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(constants::HOPR_SWARM_IDLE_CONNECTION_TIMEOUT))
        .build())
}

pub type HoprSwarm = libp2p::Swarm<HoprNetworkBehavior>;
