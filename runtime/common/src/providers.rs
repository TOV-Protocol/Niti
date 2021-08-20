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

use crate::AccountId;
use frame_support::traits::Get;
use sp_std::vec::Vec;

pub struct UdAccountsProvider<Runtime>(core::marker::PhantomData<Runtime>);
impl<Runtime: pallet_ud_accounts_storage::Config> Get<u64> for UdAccountsProvider<Runtime> {
    fn get() -> u64 {
        <pallet_ud_accounts_storage::Pallet<Runtime>>::ud_accounts_count()
    }
}
impl<Runtime: frame_system::Config<AccountId = AccountId> + pallet_ud_accounts_storage::Config>
    Get<Vec<AccountId>> for UdAccountsProvider<Runtime>
{
    fn get() -> Vec<AccountId> {
        <pallet_ud_accounts_storage::Pallet<Runtime>>::account_list()
    }
}