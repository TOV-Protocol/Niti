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

//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./duniter
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
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

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		(Weight::from_ref_time(694_175_000))
			// Standard Error: 0
			.saturating_add((Weight::from_ref_time(405_000)).saturating_mul(z as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		(Weight::from_ref_time(1_258_408_000))
			// Standard Error: 752_000
			.saturating_add((Weight::from_ref_time(34_672_000)).saturating_mul(s as u64))
			// Standard Error: 0
			.saturating_add((Weight::from_ref_time(249_000)).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		(Weight::from_ref_time(865_034_000))
			// Standard Error: 878_000
			.saturating_add((Weight::from_ref_time(36_962_000)).saturating_mul(s as u64))
			// Standard Error: 0
			.saturating_add((Weight::from_ref_time(250_000)).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		(Weight::from_ref_time(1_884_292_000))
			// Standard Error: 1_339_000
			.saturating_add((Weight::from_ref_time(80_505_000)).saturating_mul(s as u64))
			// Standard Error: 0
			.saturating_add((Weight::from_ref_time(980_000)).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn approve_as_multi_create(s: u32, ) -> Weight {
		(Weight::from_ref_time(1_267_276_000))
			// Standard Error: 243_000
			.saturating_add((Weight::from_ref_time(33_108_000)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:0)
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		(Weight::from_ref_time(865_683_000))
			// Standard Error: 403_000
			.saturating_add((Weight::from_ref_time(35_735_000)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn cancel_as_multi(s: u32, ) -> Weight {
		(Weight::from_ref_time(6_654_037_000))
			// Standard Error: 300_000
			.saturating_add((Weight::from_ref_time(33_665_000)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
