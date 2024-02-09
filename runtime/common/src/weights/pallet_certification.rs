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

//! Autogenerated weights for `pallet_certification`
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

/// Weight functions for `pallet_certification`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_certification::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:2 w:2)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:2 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsRemovableOn` (r:1 w:1)
	/// Proof: `Certification::CertsRemovableOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_cert() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `956`
		//  Estimated: `6896`
		// Minimum execution time: 30_158_000 picoseconds.
		Weight::from_parts(31_303_000, 0)
			.saturating_add(Weight::from_parts(0, 6896))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:1 w:1)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:2 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsRemovableOn` (r:1 w:1)
	/// Proof: `Certification::CertsRemovableOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn renew_cert() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `980`
		//  Estimated: `6920`
		// Minimum execution time: 28_884_000 picoseconds.
		Weight::from_parts(29_612_000, 0)
			.saturating_add(Weight::from_parts(0, 6920))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:2 w:2)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn del_cert() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `444`
		//  Estimated: `6384`
		// Minimum execution time: 16_455_000 picoseconds.
		Weight::from_parts(17_256_000, 0)
			.saturating_add(Weight::from_parts(0, 6384))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:1000 w:1000)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:0)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[2, 1000]`.
	fn remove_all_certs_received_by(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `546 + i * (35 ±0)`
		//  Estimated: `4017 + i * (2511 ±0)`
		// Minimum execution time: 23_417_000 picoseconds.
		Weight::from_parts(23_956_000, 0)
			.saturating_add(Weight::from_parts(0, 4017))
			// Standard Error: 22_385
			.saturating_add(Weight::from_parts(8_489_500, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 2511).saturating_mul(i.into()))
	}
	/// Storage: `Certification::CertsRemovableOn` (r:1 w:0)
	/// Proof: `Certification::CertsRemovableOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3604`
		// Minimum execution time: 2_483_000 picoseconds.
		Weight::from_parts(2_609_000, 0)
			.saturating_add(Weight::from_parts(0, 3604))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_remove_cert_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237`
		//  Estimated: `3702`
		// Minimum execution time: 3_474_000 picoseconds.
		Weight::from_parts(3_649_000, 0)
			.saturating_add(Weight::from_parts(0, 3702))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:2 w:2)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:0)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_remove_cert() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `660`
		//  Estimated: `6600`
		// Minimum execution time: 19_307_000 picoseconds.
		Weight::from_parts(20_020_000, 0)
			.saturating_add(Weight::from_parts(0, 6600))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
