


//	NEED TO BE REPLACED BY FILE GENERATED WITH REFERENCE MACHINE 




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

//! Autogenerated weights for `pallet_provide_randomness`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benjamin-xps139380`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_provide_randomness
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=runtime/common/src/weights/pallet_provide_randomness.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_provide_randomness`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_provide_randomness::WeightInfo for WeightInfo<T> {
	// Storage: ProvideRandomness CounterForRequestsIds (r:1 w:1)
	// Storage: ProvideRandomness RequestIdProvider (r:1 w:1)
	// Storage: ProvideRandomness RequestsIds (r:1 w:1)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: ProvideRandomness NexEpochHookIn (r:1 w:0)
	// Storage: ProvideRandomness RequestsReadyAtEpoch (r:1 w:1)
	fn request() -> Weight {
		// Minimum execution time: 297_491 nanoseconds.
		Weight::from_ref_time(416_014_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ProvideRandomness RequestsReadyAtNextBlock (r:1 w:1)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ProvideRandomness RequestsIds (r:1 w:1)
	// Storage: ProvideRandomness CounterForRequestsIds (r:1 w:1)
	// Storage: Account PendingRandomIdAssignments (r:1 w:0)
	// Storage: ProvideRandomness NexEpochHookIn (r:1 w:1)
	/// The range of component `i` is `[1, 100]`.
	fn on_initialize(i: u32, ) -> Weight {
		// Minimum execution time: 171_605 nanoseconds.
		Weight::from_ref_time(214_775_000 as u64)
			// Standard Error: 802_126
			.saturating_add(Weight::from_ref_time(53_240_660 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(i as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
}