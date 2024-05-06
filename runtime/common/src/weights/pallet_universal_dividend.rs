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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

/// Weight functions for `pallet_universal_dividend`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_universal_dividend::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `UniversalDividend::CurrentUdIndex` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUdIndex` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `UniversalDividend::PastReevals` (r:1 w:0)
	/// Proof: `UniversalDividend::PastReevals` (`max_values`: Some(1), `max_size`: Some(1602), added: 2097, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 160]`.
	fn claim_uds(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `643`
		//  Estimated: `4108`
		// Minimum execution time: 25_250_000 picoseconds.
		Weight::from_parts(26_478_413, 0)
			.saturating_add(Weight::from_parts(0, 4108))
			// Standard Error: 2_994
			.saturating_add(Weight::from_parts(12_798, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `UniversalDividend::CurrentUd` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUd` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn transfer_ud() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `6126`
		// Minimum execution time: 45_153_000 picoseconds.
		Weight::from_parts(46_077_000, 0)
			.saturating_add(Weight::from_parts(0, 6126))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `UniversalDividend::CurrentUd` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUd` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn transfer_ud_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `93`
		//  Estimated: `3558`
		// Minimum execution time: 30_379_000 picoseconds.
		Weight::from_parts(31_216_000, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `UniversalDividend::CurrentUdIndex` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUdIndex` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `UniversalDividend::PastReevals` (r:1 w:0)
	/// Proof: `UniversalDividend::PastReevals` (`max_values`: Some(1), `max_size`: Some(1602), added: 2097, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 160]`.
	fn on_removed_member(_i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3558`
		// Minimum execution time: 13_329_000 picoseconds.
		Weight::from_parts(14_784_356, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
