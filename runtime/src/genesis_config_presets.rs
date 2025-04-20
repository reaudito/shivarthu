// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig};
use alloc::{vec, vec::Vec};
use hex_literal::hex;
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::AccountKeyring;

// Returns the genesis config presets populated with given parameters.
fn testnet_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
) -> Value {
    let config = RuntimeGenesisConfig {
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1u128 << 60))
                .collect::<Vec<_>>(),
        },
        aura: pallet_aura::GenesisConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.0.clone()))
                .collect::<Vec<_>>(),
        },
        grandpa: pallet_grandpa::GenesisConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect::<Vec<_>>(),
            ..Default::default()
        },
        sudo: SudoConfig { key: Some(root) },
        ..Default::default()
    };

    serde_json::to_value(config).expect("Could not build genesis config.")
}

/// Return the development genesis config.
pub fn development_config_genesis() -> Value {
    testnet_genesis(
        vec![(
            sp_keyring::Sr25519Keyring::Alice.public().into(),
            sp_keyring::Ed25519Keyring::Alice.public().into(),
        )],
        vec![
            AccountKeyring::Alice.to_account_id(),
            AccountKeyring::Bob.to_account_id(),
            AccountKeyring::AliceStash.to_account_id(),
            AccountKeyring::BobStash.to_account_id(),
            hex!("2e778beae3cc11fd7ea694f4ff8b54922d67e0599c356f393277ed9711d6364b").into(),
            hex!("2e1c14cd13a2b090a62203809d8ce3eaac7417a4a0272438568eb04cae330669").into(),
            hex!("ba0ce278d82ef9a686cb60a801125a8d11b32caa2456ebdcfe7ff687bb9bf540").into(),
            hex!("600f10bdbf233ac6614eea62ae45d269b43c759e4ddf0bc1a70ffcbc95499c6c").into(),
            hex!("c2da35a7aed402249295971abe8f10e0b03d861a0571e56115bcc6f8828dd939").into(),
            hex!("186863b612097dec4ce7b9772381935baa7fc6dc7c44695f0384174f1b131156").into(),
            hex!("70c3f87a26743fed9194f8fc67bfdd9a211f3b00f5c80459107022d096dbf928").into(),
            hex!("cab4abef5dda97cc98eb0f3a5e0329bd2c1b892b5f442021a634c7e79e6f6e29").into(),
            hex!("ac926b4e81989ca51c9ac6f0ef9c7db08d5334bb0a5c3b0194bf92d215b50f3f").into(),
            hex!("186c72f04de9c1a74cee6836c08b6d56a88e90ab5a6127693a55379e8e03d919").into(),
            hex!("b02de28c52fe59f9a3d8779cd8c6ee7439cba45e48e7ee582f5cc939c7b5946c").into(),
        ],
        sp_keyring::AccountKeyring::Alice.to_account_id(),
    )
}

/// Return the local genesis config preset.
pub fn local_config_genesis() -> Value {
    testnet_genesis(
        vec![
            (
                sp_keyring::Sr25519Keyring::Alice.public().into(),
                sp_keyring::Ed25519Keyring::Alice.public().into(),
            ),
            (
                sp_keyring::Sr25519Keyring::Bob.public().into(),
                sp_keyring::Ed25519Keyring::Bob.public().into(),
            ),
        ],
        AccountKeyring::iter()
            .filter(|v| v != &AccountKeyring::One && v != &AccountKeyring::Two)
            .map(|v| v.to_account_id())
            .collect::<Vec<_>>(),
        AccountKeyring::Alice.to_account_id(),
    )
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
    let patch = match id.as_ref() {
        sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
        sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
        _ => return None,
    };
    Some(
        serde_json::to_string(&patch)
            .expect("serialization to json is expected to work. qed.")
            .into_bytes(),
    )
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
    vec![
        PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
        PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
    ]
}
