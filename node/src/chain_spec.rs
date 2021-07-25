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

pub mod g1;
pub mod gdev;
pub mod gtest;

use common_runtime::IdtyIndex;
use common_runtime::{
    entities::{IdtyDid, Planet},
    AccountId, Signature,
};
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::{BTreeMap, BTreeSet};

pub type AccountPublic = <Signature as Verify>::Signer;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/*/// Generate an account ID from pain.
pub fn get_account_id_from_pair<TPublic: Public>(pair: TPublic::Pair) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(pair.public()).into_account()
}*/

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn clique_wot(initial_identities_len: usize) -> BTreeMap<IdtyIndex, BTreeSet<IdtyIndex>> {
    let mut certs_by_issuer = BTreeMap::new();
    for i in 1..=initial_identities_len {
        certs_by_issuer.insert(
            i as IdtyIndex,
            (1..=initial_identities_len)
                .filter_map(|j| if i != j { Some(j as IdtyIndex) } else { None })
                .collect(),
        );
    }
    certs_by_issuer
}

/// Create a fake did (for dev and testnet)
fn did(u8_: u8) -> IdtyDid {
    IdtyDid {
        hash: sp_core::H256::repeat_byte(u8_),
        planet: Planet::Earth,
        latitude: 0,
        longitude: 0,
    }
}
