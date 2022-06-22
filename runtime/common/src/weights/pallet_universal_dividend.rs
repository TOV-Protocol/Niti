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

//! Autogenerated weights for `pallet_universal_dividend`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./duniter
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_universal_dividend
// --extrinsic=*
// --execution=wasm
// --wasm-execution=interpreted-i-know-what-i-do
// --heap-pages=4096
// --header=./file_header.txt
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_universal_dividend`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_universal_dividend::WeightInfo for WeightInfo<T> {
	// Storage: Parameters ParametersStorage (r:1 w:0)
	fn on_initialize() -> Weight {
		(104_055_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: UniversalDividend CurrentUd (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Account PendingNewAccounts (r:0 w:1)
	fn transfer_ud() -> Weight {
		(2_468_842_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: UniversalDividend CurrentUd (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Account PendingNewAccounts (r:0 w:1)
	fn transfer_ud_keep_alive() -> Weight {
		(1_442_150_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
