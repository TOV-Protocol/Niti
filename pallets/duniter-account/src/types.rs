// Copyright 2021 Axiom-Team
//
// This file is part of Duniter-v2S.
//
// Duniter-v2S is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Duniter-v2S is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Duniter-v2S. If not, see <https://www.gnu.org/licenses/>.

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::Zero;

// see `struct AccountData` for details in substrate code
#[derive(Clone, Decode, Encode, Eq, MaxEncodedLen, PartialEq, RuntimeDebug, TypeInfo)] // Default,
pub struct AccountData<Balance, IdtyId> {
    /// A random identifier that can not be chosen by the user
    // this intends to be used as a robust identification system
    pub(super) random_id: Option<H256>,
    // see Substrate AccountData
    pub(super) free: Balance,
    // see Substrate AccountData
    pub(super) reserved: Balance,
    // see Substrate AccountData
    fee_frozen: Balance,
    /// an optional pointer to an identity
    // used to know if this account is linked to a member
    // used in quota system to refund fees
    pub linked_idty: Option<IdtyId>,
}

// explicit implementation of default trait (can not be derived)
impl<Balance: Zero, IdtyId> Default for AccountData<Balance, IdtyId> {
    fn default() -> Self {
        Self {
            linked_idty: None,
            random_id: None,
            free: Balance::zero(),
            reserved: Balance::zero(),
            fee_frozen: Balance::zero(),
        }
    }
}

impl<Balance: Zero, IdtyId> AccountData<Balance, IdtyId> {
    pub fn set_balances(&mut self, new_balances: pallet_balances::AccountData<Balance>) {
        self.free = new_balances.free;
        self.reserved = new_balances.reserved;
        self.fee_frozen = new_balances.frozen;
    }
}

// convert Duniter AccountData to Balances AccountData
// needed for trait implementation
impl<Balance: Zero, IdtyId> From<AccountData<Balance, IdtyId>>
    for pallet_balances::AccountData<Balance>
{
    fn from(account_data: AccountData<Balance, IdtyId>) -> Self {
        Self {
            free: account_data.free,
            reserved: account_data.reserved,
            frozen: account_data.fee_frozen,
            flags: Default::default(), // default flags since not used
        }
    }
}

#[derive(
    Clone,
    Decode,
    Default,
    Encode,
    Eq,
    MaxEncodedLen,
    PartialEq,
    RuntimeDebug,
    TypeInfo,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct GenesisAccountData<Balance, IdtyId> {
    pub random_id: H256,
    pub balance: Balance,
    pub idty_id: Option<IdtyId>,
}
