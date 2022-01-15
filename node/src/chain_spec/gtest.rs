// Copyright 2021 Axiom-Team
//
// This file is part of Substrate-Libre-Currency.
//
// Substrate-Libre-Currency is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Substrate-Libre-Currency is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Substrate-Libre-Currency. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use common_runtime::constants::*;
use common_runtime::entities::IdtyName;
use gtest_runtime::{
    opaque::SessionKeys, AccountId, BabeConfig, BalancesConfig, GenesisConfig, IdentityConfig,
    IdtyRight, IdtyValue, ImOnlineId, SessionConfig, StrongCertConfig, SudoConfig, SystemConfig,
    UdAccountsStorageConfig, UniversalDividendConfig, WASM_BINARY,
};
use maplit::btreemap;
use sc_service::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use std::collections::BTreeMap;

pub type AuthorityKeys = (
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    AuthorityDiscoveryId,
);

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

const TOKEN_DECIMALS: usize = 2;
const TOKEN_SYMBOL: &str = "ĞT";
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Generate an authority keys.
pub fn get_authority_keys_from_seed(s: &str) -> AuthorityKeys {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
        get_from_seed::<AuthorityDiscoveryId>(s),
    )
}

fn devnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<AuthorityKeys>,
    initial_identities: BTreeMap<IdtyName, AccountId>,
    root_key: AccountId,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        authority_discovery: Default::default(),
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of INITIAL_BALANCE.
            balances: Vec::with_capacity(0),
        },
        babe: BabeConfig {
            authorities: Vec::with_capacity(0),
            epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
        },
        grandpa: Default::default(),
        im_online: Default::default(),
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.1.clone(), x.2.clone(), x.3.clone(), x.4.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        identity: IdentityConfig {
            identities: initial_identities
                .iter()
                .map(|(name, account)| IdtyValue {
                    name: name.clone(),
                    expire_on: gtest_runtime::MaxInactivityPeriod::get(),
                    owner_key: account.clone(),
                    removable_on: 0,
                    renewable_on: gtest_runtime::StrongCertRenewablePeriod::get(),
                    rights: vec![
                        (IdtyRight::CreateIdty, None),
                        (IdtyRight::StrongCert, None),
                        (IdtyRight::Ud, None),
                    ],
                    status: gtest_runtime::IdtyStatus::Validated,
                    data: Default::default(),
                })
                .collect(),
        },
        strong_cert: StrongCertConfig {
            certs_by_issuer: clique_wot(
                initial_identities.len(),
                gtest_runtime::parameters::ValidityPeriod::get(),
            ),
        },
        ud_accounts_storage: UdAccountsStorageConfig {
            ud_accounts: initial_identities.values().cloned().collect(),
        },
        universal_dividend: UniversalDividendConfig {
            first_ud: 1_000,
            initial_monetary_mass: 0,
        },
    }
}

pub fn development_chain_spec() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Ğtest Development",
        // ID
        "gtest_dev",
        ChainType::Development,
        move || {
            devnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![get_authority_keys_from_seed("Alice")],
                // Inital identities
                btreemap![
                    IdtyName::from("Alice") => get_account_id_from_seed::<sr25519::Public>("Alice"),
                    IdtyName::from("Bob") => get_account_id_from_seed::<sr25519::Public>("Bob"),
                    IdtyName::from("Charlie") => get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(
            serde_json::json!({
                    "tokenDecimals": TOKEN_DECIMALS,
                    "tokenSymbol": TOKEN_SYMBOL,
            })
            .as_object()
            .expect("must be a map")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config(authorities_count: usize) -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "wasm not available".to_string())?;

    let mut authorities = vec![
        get_authority_keys_from_seed("Alice"),
        get_authority_keys_from_seed("Bob"),
        get_authority_keys_from_seed("Charlie"),
        get_authority_keys_from_seed("Dave"),
        get_authority_keys_from_seed("Eve"),
        get_authority_keys_from_seed("Ferdie"),
    ];
    authorities.truncate(authorities_count);

    Ok(ChainSpec::from_genesis(
        // Name
        "Ğtest Local Testnet",
        // ID
        "gtest_local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial authorities
                authorities.clone(),
                // Initial identities
                btreemap![
                    IdtyName::from("Alice") => get_account_id_from_seed::<sr25519::Public>("Alice"),
                    IdtyName::from("Bob") => get_account_id_from_seed::<sr25519::Public>("Bob"),
                    IdtyName::from("Charlie") => get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    IdtyName::from("Dave") => get_account_id_from_seed::<sr25519::Public>("Dave"),
                    IdtyName::from("Eve") => get_account_id_from_seed::<sr25519::Public>("Eve"),
                    IdtyName::from("Ferdie") => get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(
            serde_json::json!({
                    "tokenDecimals": TOKEN_DECIMALS,
                    "tokenSymbol": TOKEN_SYMBOL,
            })
            .as_object()
            .expect("must be a map")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

fn session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        babe,
        grandpa,
        im_online,
        authority_discovery,
    }
}

fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<AuthorityKeys>,
    initial_identities: BTreeMap<IdtyName, AccountId>,
    root_key: AccountId,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        authority_discovery: Default::default(),
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of INITIAL_BALANCE.
            balances: Vec::with_capacity(0),
        },
        babe: BabeConfig {
            authorities: Vec::with_capacity(0),
            epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
        },
        grandpa: Default::default(),
        im_online: Default::default(),
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.1.clone(), x.2.clone(), x.3.clone(), x.4.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        identity: IdentityConfig {
            identities: initial_identities
                .iter()
                .map(|(name, account)| IdtyValue {
                    name: name.clone(),
                    expire_on: gtest_runtime::MaxInactivityPeriod::get(),
                    owner_key: account.clone(),
                    removable_on: 0,
                    renewable_on: gtest_runtime::StrongCertRenewablePeriod::get(),
                    rights: vec![
                        (IdtyRight::CreateIdty, None),
                        (IdtyRight::StrongCert, None),
                        (IdtyRight::Ud, None),
                    ],
                    status: gtest_runtime::IdtyStatus::Validated,
                    data: Default::default(),
                })
                .collect(),
        },
        strong_cert: StrongCertConfig {
            certs_by_issuer: clique_wot(
                initial_identities.len(),
                gtest_runtime::parameters::ValidityPeriod::get(),
            ),
        },
        ud_accounts_storage: UdAccountsStorageConfig {
            ud_accounts: initial_identities.values().cloned().collect(),
        },
        universal_dividend: UniversalDividendConfig {
            first_ud: 1_000,
            initial_monetary_mass: 0,
        },
    }
}
