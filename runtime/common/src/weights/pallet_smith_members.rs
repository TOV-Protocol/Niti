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

//! Autogenerated weights for `pallet_smith_members`
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
// --execution=wasm
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

/// Weight functions for `pallet_smith_members`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_smith_members::WeightInfo for WeightInfo<T> {
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembers Smiths (r:2 w:1)
	/// Proof Skipped: SmithMembers Smiths (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:0)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembers CurrentSession (r:1 w:0)
	/// Proof Skipped: SmithMembers CurrentSession (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SmithMembers ExpiresOn (r:1 w:1)
	/// Proof Skipped: SmithMembers ExpiresOn (max_values: None, max_size: None, mode: Measured)
	fn invite_smith() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `732`
		//  Estimated: `6672`
		// Minimum execution time: 61_398_000 picoseconds.
		Weight::from_parts(63_088_000, 0)
			.saturating_add(Weight::from_parts(0, 6672))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembers Smiths (r:1 w:1)
	/// Proof Skipped: SmithMembers Smiths (max_values: None, max_size: None, mode: Measured)
	fn accept_invitation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3928`
		// Minimum execution time: 31_273_000 picoseconds.
		Weight::from_parts(32_005_000, 0)
			.saturating_add(Weight::from_parts(0, 3928))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembers Smiths (r:2 w:2)
	/// Proof Skipped: SmithMembers Smiths (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SmithMembers CurrentSession (r:1 w:0)
	/// Proof Skipped: SmithMembers CurrentSession (max_values: Some(1), max_size: None, mode: Measured)
	fn certify_smith() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `577`
		//  Estimated: `6517`
		// Minimum execution time: 47_601_000 picoseconds.
		Weight::from_parts(48_286_000, 0)
			.saturating_add(Weight::from_parts(0, 6517))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
