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

//! Autogenerated weights for `pallet_distance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
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

/// Weight functions for `pallet_distance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_distance::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::PendingEvaluationRequest` (r:1 w:1)
	/// Proof: `Distance::PendingEvaluationRequest` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:0)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:1 w:0)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::EvaluationPool2` (r:1 w:1)
	/// Proof: `Distance::EvaluationPool2` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn request_distance_evaluation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1596`
		//  Estimated: `5061`
		// Minimum execution time: 41_417_000 picoseconds.
		Weight::from_parts(43_366_000, 0)
			.saturating_add(Weight::from_parts(0, 5061))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:2 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::PendingEvaluationRequest` (r:1 w:1)
	/// Proof: `Distance::PendingEvaluationRequest` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:0)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:1 w:0)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::EvaluationPool2` (r:1 w:1)
	/// Proof: `Distance::EvaluationPool2` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn request_distance_evaluation_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1625`
		//  Estimated: `7565`
		// Minimum execution time: 43_523_000 picoseconds.
		Weight::from_parts(45_745_000, 0)
			.saturating_add(Weight::from_parts(0, 7565))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Distance::DidUpdate` (r:1 w:1)
	/// Proof: `Distance::DidUpdate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Authorship::Author` (r:1 w:1)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::EvaluationPool0` (r:1 w:1)
	/// Proof: `Distance::EvaluationPool0` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[1, 600]`.
	fn update_evaluation(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `773 + i * (10 ±0)`
		//  Estimated: `2257 + i * (10 ±0)`
		// Minimum execution time: 13_633_000 picoseconds.
		Weight::from_parts(16_097_962, 0)
			.saturating_add(Weight::from_parts(0, 2257))
			// Standard Error: 139
			.saturating_add(Weight::from_parts(103_266, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(i.into()))
	}
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Distance::EvaluationPool0` (r:1 w:1)
	/// Proof: `Distance::EvaluationPool0` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[1, 600]`.
	fn force_update_evaluation(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612 + i * (10 ±0)`
		//  Estimated: `2096 + i * (10 ±0)`
		// Minimum execution time: 6_222_000 picoseconds.
		Weight::from_parts(8_460_482, 0)
			.saturating_add(Weight::from_parts(0, 2096))
			// Standard Error: 296
			.saturating_add(Weight::from_parts(104_396, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(i.into()))
	}
	/// Storage: `Identity::Identities` (r:1 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::MembershipsExpireOn` (r:2 w:2)
	/// Proof: `Membership::MembershipsExpireOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_valid_distance_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `609`
		//  Estimated: `6549`
		// Minimum execution time: 24_403_000 picoseconds.
		Weight::from_parts(25_562_000, 0)
			.saturating_add(Weight::from_parts(0, 6549))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Distance::DidUpdate` (r:1 w:1)
	/// Proof: `Distance::DidUpdate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `1655`
		// Minimum execution time: 2_756_000 picoseconds.
		Weight::from_parts(2_960_000, 0)
			.saturating_add(Weight::from_parts(0, 1655))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
