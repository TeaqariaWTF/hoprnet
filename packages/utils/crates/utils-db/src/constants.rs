pub const LATEST_BLOCK_NUMBER_KEY: &str = "latestBlockNumber";
pub const PACKET_TAG_BLOOM_FILTER: &str = "packetTagBloomFilter";
pub const ACCOUNT_PREFIX: &str = "account-";
pub const CHANNEL_PREFIX: &str = "channel-";
pub const CHANNELS_DOMAIN_SEPARATOR_KEY: &str = "channels:domainSeparator";
pub const CHANNELS_LEDGER_DOMAIN_SEPARATOR_KEY: &str = "channels:ledgerDomainSeparator";
pub const COMMITMENT_PREFIX: &str = "commitment-";
pub const CURRENT_COMMITMENT_PREFIX: &str = "commitment:current-";
pub const TICKET_INDEX_PREFIX: &str = "ticketIndex-";
pub const REJECTED_TICKETS_COUNT: &str = "statistics:rejected:count";
pub const REJECTED_TICKETS_VALUE: &str = "statistics:rejected:value";
pub const REDEEMED_TICKETS_COUNT: &str = "statistics:redeemed:count";
pub const REDEEMED_TICKETS_VALUE: &str = "statistics:redeemed:value";
pub const NEGLECTED_TICKETS_COUNT: &str = "statistics:neglected:count";
pub const NEGLECTED_TICKETS_VALUE: &str = "statistics:neglected:value";
pub const LOSING_TICKET_COUNT: &str = "statistics:losing:count";
pub const LATEST_CONFIRMED_SNAPSHOT_KEY: &str = "latestConfirmedSnapshot";
pub const PENDING_ACKNOWLEDGEMENTS_PREFIX: &str = "tickets:pending-acknowledgement-";
pub const ACKNOWLEDGED_TICKETS_PREFIX: &str = "tickets:acknowledged-";
pub const HOPR_BALANCE_KEY: &str = "hopr-balance";
pub const TICKET_PRICE_KEY: &str = "ticket-price";
pub const STAKING_SAFE_ADDRESS_KEY: &str = "staking:safe-address";
pub const STAKING_MODULE_ADDRESS_KEY: &str = "staking:module-address";
pub const STAKING_SAFE_ALLOWANCE_KEY: &str = "staking:safe-allowance";
pub const NODE_SAFE_REGISTRY_DOMAIN_SEPARATOR_KEY: &str = "nodeSafeRegistry:domainSeparator";
pub const NETWORK_REGISTRY_ALLOWED_PREFIX: &str = "networkRegistry:allowed-";
pub const NETWORK_REGISTRY_ENABLED_PREFIX: &str = "networkRegistry:enabled";
pub const NETWORK_REGISTRY_ADDRESS_ELIGIBLE_PREFIX: &str = "networkRegistry:addressEligible-";
pub const NETWORK_REGISTRY_ADDRESS_CHAIN_KEY_PREFIX: &str = "networkRegistry:addressPublicKey-";
/// some Multi-Factor Authorization module, e.g. Gnosis Safe
pub const MFA_MODULE_PREFIX: &str = "mfaModule:enabled";
pub const CHAIN_KEY_PREFIX: &str = "keys:chain-";
pub const PACKET_KEY_PREFIX: &str = "keys:packet-";
