// Copyright 2021-2022 Axiom-Team
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

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-22, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `squirrel`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=2
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Account::PendingNewAccounts` (r:0 w:1)
	/// Proof: `Account::PendingNewAccounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3591`
		// Minimum execution time: 103_052_000 picoseconds.
		Weight::from_parts(104_632_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Account PendingNewAccounts (r:0 w:1)
	/// Proof Skipped: Account PendingNewAccounts (max_values: None, max_size: None, mode: Measured)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3591`
		// Minimum execution time: 133_263_000 picoseconds.
		Weight::from_parts(136_924_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Account PendingNewAccounts (r:0 w:1)
	/// Proof Skipped: Account PendingNewAccounts (max_values: None, max_size: None, mode: Measured)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64`
		//  Estimated: `6192`
		// Minimum execution time: 105_518_000 picoseconds.
		Weight::from_parts(226_206_000, 0)
			.saturating_add(Weight::from_parts(0, 6192))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3591`
		// Minimum execution time: 9_693_000 picoseconds.
		Weight::from_parts(10_211_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3591`
		// Minimum execution time: 14_040_000 picoseconds.
		Weight::from_parts(14_480_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Account::PendingNewAccounts` (r:0 w:1)
	/// Proof: `Account::PendingNewAccounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3591`
		// Minimum execution time: 38_912_000 picoseconds.
		Weight::from_parts(40_985_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3591`
		// Minimum execution time: 41_380_000 picoseconds.
		Weight::from_parts(44_612_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
