//! Network and physical transport related utilities
//!
//! This package contains objects and utilities that allow storing and
//! querying the current state of the observable `Network`.
//!
//! Background processes and event updates ensure that the Network data is
//! regularly updated to offer the complete information about each observed
//! peer:
//! 1. the timestamps of last observations
//! 2. quality measurements based on the heartbeat and ping reporting mechanism,
//! 3. individual metrics counting windowed quality characteristics
//! 4. on and off-chain observed peer related data (multiaddresses...)
//!
//! NOTE: This object has not been significantly altered from the previous versions
//! and will be improved in the upcoming releases together with physical transport
//! upgrades that will make it possible to record more data faster. As such, this
//! object should slowly transform into a pollable physical network graph processing
//! live telemetry from the packet transport process.

/// Configuration of the network module.
pub mod config;

/// Global constants published from this crate.
pub mod constants;
/// Errors that can be generated by the crate.
pub mod errors;
/// Implementation of the main loop for the heartbeat mechanism.
pub mod heartbeat;
/// Low-level transport protocol messaging definitions for [ping].
pub mod messaging;
/// Implementation of the peer network logic
pub mod network;
/// Implementation of the pinging mechanism used by the [heartbeat] and manual pings.
pub mod ping;

pub use hopr_db_api::peers::HoprDbPeersOperations;
pub use libp2p_identity::PeerId;
