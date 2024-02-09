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

/// Global constants published from this crate.
pub mod constants;
/// Enumerates all errors in this crate.
pub mod errors;
/// Implementation of the main loop for the heartbeat mechanism.
pub mod heartbeat;
/// Contains low-level transport protocol messaging definitions for [ping].
pub mod messaging;
/// Implements the peer network logic
pub mod network;
/// Implements the pinging mechanism used by [heartbeat] and manual pings.
pub mod ping;

pub use libp2p_identity::PeerId;
