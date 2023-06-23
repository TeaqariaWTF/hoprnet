use real_base::real;
use serde::{Deserialize, Serialize};
use utils_misc::ok_or_str;
// import need to be called wasm_bindgen to make field annotations
// such as `#[wasm_bindgen(skip)]` work
use utils_proc_macros::wasm_bindgen_if as wasm_bindgen;

#[cfg(any(not(feature = "wasm"), test))]
use real_base::file::native::read_file;
#[cfg(all(feature = "wasm", not(test)))]
use real_base::file::wasm::read_file;

pub trait FromJsonFile: Sized {
    fn from_json_file(mono_repo_path: &str) -> Result<Self, String>;
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all(deserialize = "lowercase"))]
#[wasm_bindgen]
pub enum EnvironmentType {
    Production,
    Staging,
    Development,
    Local,
}

impl ToString for EnvironmentType {
    fn to_string(&self) -> String {
        match self {
            Self::Production => "production".into(),
            Self::Staging => "staging".into(),
            Self::Development => "development".into(),
            Self::Local => "local".into(),
        }
    }
}

/// Holds all information we need about the blockchain network
/// the client is going to use
#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[wasm_bindgen(getter_with_clone)]
pub struct ChainOptions {
    #[serde(skip_deserializing)]
    pub id: String,
    pub description: String,
    /// >= 0
    pub chain_id: u32,
    pub live: bool,
    /// a valid HTTP url pointing at a RPC endpoint
    pub default_provider: String,
    /// a valid HTTP url pointing at a RPC endpoint
    pub etherscan_api_url: Option<String>,
    /// The absolute maximum you are willing to pay per unit of gas to get your transaction included in a block, e.g. '10 gwei'
    pub max_fee_per_gas: String,
    /// Tips paid directly to miners, e.g. '2 gwei'
    pub max_priority_fee_per_gas: String,
    pub native_token_name: String,
    pub hopr_token_name: String,
    #[wasm_bindgen(skip)] // no tags in Typescript
    pub tags: Option<Vec<String>>,
}

/// Holds all information about the protocol network
/// to be used by the client
#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[wasm_bindgen(getter_with_clone)]
pub struct Network {
    #[serde(skip_deserializing)]
    pub id: String,
    /// must match one of the Network.id
    pub chain: String,
    pub environment_type: EnvironmentType,
    // Node.js-fashioned semver string
    pub version_range: String,
    pub indexer_start_block_number: u32,
    /// an Ethereum address
    pub token_contract_address: String,
    /// an Ethereum address
    pub channels_contract_address: String,
    /// an Ethereum address
    pub xhopr_contract_address: String,
    /// an Ethereum address
    pub boost_contract_address: String,
    /// an Ethereum address
    pub stake_contract_address: String,
    /// an Ethereum address
    pub network_registry_proxy_contract_address: String,
    /// an Ethereum address
    pub network_registry_contract_address: String,
    #[wasm_bindgen(skip)] // no tags in Typescript
    pub tags: Vec<String>,
    /// the associated staking season
    pub stake_season: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolConfig {
    pub networks: std::collections::HashMap<String, Network>,
    pub chains: std::collections::HashMap<String, ChainOptions>,
}

impl FromJsonFile for ProtocolConfig {
    /// Reads the protocol config JSON file and returns it
    fn from_json_file(mono_repo_path: &str) -> Result<Self, String> {
        let protocol_config_path = format!("{}/packages/core/protocol-config.json", mono_repo_path);

        let data = ok_or_str!(read_file(protocol_config_path.as_str()))?;

        let mut protocol_config = ok_or_str!(serde_json::from_slice::<ProtocolConfig>(&data))?;

        for (id, env) in protocol_config.networks.iter_mut() {
            env.id = id.to_owned();
        }

        for (id, network) in protocol_config.networks.iter_mut() {
            network.id = id.to_owned();
        }

        Ok(protocol_config)
    }
}

impl ProtocolConfig {
    /// Returns a list of environments which the node is able to work with
    pub fn supported_networks(&self, mono_repo_path: &str) -> Result<Vec<Network>, String> {
        let version = PackageJsonFile::from_json_file(&mono_repo_path)
            .and_then(|p| ok_or_str!(real::coerce_version(p.version.as_str())))?;

        let mut allowed: Vec<Network> = vec![];

        for (_, env) in self.networks.iter() {
            let range = env.version_range.to_owned();

            if let Ok(true) = real::satisfies(version.as_str(), range.as_str()) {
                allowed.push(env.to_owned())
            }
        }

        Ok(allowed)
    }
}

#[derive(Deserialize)]
pub struct PackageJsonFile {
    version: String,
}

impl FromJsonFile for PackageJsonFile {
    fn from_json_file(mono_repo_path: &str) -> Result<Self, String> {
        let package_json_path = format!("{}/packages/hoprd/package.json", mono_repo_path);

        let data = ok_or_str!(read_file(package_json_path.as_str()))?;

        ok_or_str!(serde_json::from_slice::<PackageJsonFile>(&data))
    }
}

impl PackageJsonFile {
    pub fn coerced_version(&self) -> Result<String, String> {
        /*
         * Coerced full version using
         * coerce_version('42.6.7.9.3-alpha') // '42.6.7'
         */
        ok_or_str!(real::coerce_version(self.version.as_str()))
    }
}

#[derive(Serialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct ResolvedNetwork {
    /// the network identifier, e.g. monte_rosa
    pub id: String,
    pub chain: ChainOptions,
    pub environment_type: EnvironmentType,
    /// an Ethereum address
    pub channels_contract_address: String,
    pub channel_contract_deploy_block: u32,
    /// an Ethereum address
    pub token_contract_address: String,
    /// an Ethereum address
    pub xhopr_contract_address: String,
    /// an Ethereum address
    pub boost_contract_address: String,
    /// an Ethereum address
    pub stake_contract_address: String,
    /// an Ethereum address
    pub network_registry_proxy_contract_address: String,
    /// an Ethereum address
    pub network_registry_contract_address: String,
}

impl ResolvedNetwork {
    /// Returns the network details, returns an error if network is not supported
    pub fn new(mono_repo_path: &str, id: &str, maybe_custom_provider: Option<&str>) -> Result<Self, String> {
        let mut protocol_config = ProtocolConfig::from_json_file(mono_repo_path)?;
        let version = PackageJsonFile::from_json_file(mono_repo_path).and_then(|c| c.coerced_version())?;

        let network = protocol_config
            .networks
            .get_mut(id)
            .ok_or(format!("Could not find network {} in protocol config", id))?;

        let chain = protocol_config
            .chains
            .get_mut(&network.chain)
            .ok_or(format!("Invalid chain {} for network {}", network.chain, id))?;

        if let Some(custom_provider) = maybe_custom_provider {
            chain.default_provider = custom_provider.into();
        }

        match real::satisfies(version.as_str(), network.version_range.as_str()) {
            Ok(true) => Ok(ResolvedNetwork {
                id: id.into(),
                chain: chain.to_owned(),
                environment_type: network.environment_type,
                channel_contract_deploy_block: network.indexer_start_block_number,
                token_contract_address: network.token_contract_address.to_owned(),
                channels_contract_address: network.channels_contract_address.to_owned(),
                xhopr_contract_address: network.xhopr_contract_address.to_owned(),
                boost_contract_address: network.boost_contract_address.to_owned(),
                stake_contract_address: network.stake_contract_address.to_owned(),
                network_registry_contract_address: network.network_registry_contract_address.to_owned(),
                network_registry_proxy_contract_address: network.network_registry_proxy_contract_address.to_owned(),
            }),
            Ok(false) => protocol_config.supported_networks(mono_repo_path).and_then(|envs| {
                Err(format!(
                    "network {} is not supported, supported networks {:?}",
                    id,
                    envs.iter().map(|e| e.id.to_owned()).collect::<Vec<String>>().join(", ")
                ))
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::FromJsonFile;
    use utils_misc::{clean_mono_repo_path, ok_or_jserr};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;

    pub type JsResult<T> = Result<T, JsValue>;

    #[wasm_bindgen]
    pub fn supported_networks(mono_repo_path: &str) -> JsResult<JsValue> {
        clean_mono_repo_path!(mono_repo_path, cleaned_mono_repo_path);

        let supported_envs = super::ProtocolConfig::from_json_file(cleaned_mono_repo_path)
            .and_then(|c| c.supported_networks(cleaned_mono_repo_path))?;

        ok_or_jserr!(serde_wasm_bindgen::to_value(&supported_envs))
    }

    #[wasm_bindgen]
    pub fn resolve_network(mono_repo_path: &str, id: &str, maybe_custom_provider: Option<String>) -> JsResult<JsValue> {
        clean_mono_repo_path!(mono_repo_path, cleaned_mono_repo_path);

        let resolved_environment = super::ResolvedNetwork::new(
            cleaned_mono_repo_path,
            id,
            maybe_custom_provider.as_ref().map(|c| c.as_str()),
        )?;

        ok_or_jserr!(serde_wasm_bindgen::to_value(&resolved_environment))
    }
}
