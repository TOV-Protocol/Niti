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

//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet-identity
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::IdentityIndexOf` (r:2 w:1)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:2 w:2)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:2 w:2)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Identity::NextIdtyIndex` (r:1 w:1)
	/// Proof: `Identity::NextIdtyIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:1 w:1)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::CounterForIdentities` (r:1 w:1)
	/// Proof: `Identity::CounterForIdentities` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Certification::CertsRemovableOn` (r:1 w:1)
	/// Proof: `Certification::CertsRemovableOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn create_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1070`
		//  Estimated: `7010`
		// Minimum execution time: 64_979_000 picoseconds.
		Weight::from_parts(66_407_000, 0)
			.saturating_add(Weight::from_parts(0, 7010))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentitiesNames` (r:1 w:1)
	/// Proof: `Identity::IdentitiesNames` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:2 w:2)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn confirm_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `698`
		//  Estimated: `6638`
		// Minimum execution time: 28_030_000 picoseconds.
		Weight::from_parts(29_442_000, 0)
			.saturating_add(Weight::from_parts(0, 6638))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Identity::IdentityIndexOf` (r:2 w:2)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::BlockHash` (r:1 w:0)
	/// Proof: `System::BlockHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn change_owner_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `723`
		//  Estimated: `6663`
		// Minimum execution time: 73_717_000 picoseconds.
		Weight::from_parts(75_195_000, 0)
			.saturating_add(Weight::from_parts(0, 6663))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::BlockHash` (r:1 w:0)
	/// Proof: `System::BlockHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:2 w:2)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `664`
		//  Estimated: `6604`
		// Minimum execution time: 64_566_000 picoseconds.
		Weight::from_parts(65_978_000, 0)
			.saturating_add(Weight::from_parts(0, 6604))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Identity::IdentitiesNames` (r:0 w:999)
	/// Proof: `Identity::IdentitiesNames` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[2, 1000]`.
	fn prune_item_identities_names(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_611_000 picoseconds.
		Weight::from_parts(3_788_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1_076
			.saturating_add(Weight::from_parts(1_174_325, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn fix_sufficients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `104`
		//  Estimated: `3558`
		// Minimum execution time: 6_271_000 picoseconds.
		Weight::from_parts(6_529_000, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityIndexOf` (r:1 w:0)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::BlockHash` (r:1 w:0)
	/// Proof: `System::BlockHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	fn link_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403`
		//  Estimated: `3868`
		// Minimum execution time: 50_535_000 picoseconds.
		Weight::from_parts(51_906_000, 0)
			.saturating_add(Weight::from_parts(0, 3868))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 83_000 picoseconds.
		Weight::from_parts(107_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Identity::Identities` (r:1 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_revoke_identity_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3767`
		// Minimum execution time: 3_424_000 picoseconds.
		Weight::from_parts(3_629_000, 0)
			.saturating_add(Weight::from_parts(0, 3767))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:2 w:2)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::CounterForMembership` (r:1 w:1)
	/// Proof: `Membership::CounterForMembership` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Membership::MembershipsExpireOn` (r:1 w:1)
	/// Proof: `Membership::MembershipsExpireOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `UniversalDividend::CurrentUdIndex` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUdIndex` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `SmithMembers::Smiths` (r:3 w:3)
	/// Proof: `SmithMembers::Smiths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::Members` (r:1 w:1)
	/// Proof: `AuthorityMembers::Members` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OnlineAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OnlineAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OutgoingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OutgoingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::IncomingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::IncomingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Session::KeyOwner` (r:0 w:4)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1555`
		//  Estimated: `9970`
		// Minimum execution time: 81_169_000 picoseconds.
		Weight::from_parts(83_252_000, 0)
			.saturating_add(Weight::from_parts(0, 9970))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(20))
	}
	/// Storage: `Identity::Identities` (r:1 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_remove_identity_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3767`
		// Minimum execution time: 3_406_000 picoseconds.
		Weight::from_parts(3_698_000, 0)
			.saturating_add(Weight::from_parts(0, 3767))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::CounterForIdentities` (r:1 w:1)
	/// Proof: `Identity::CounterForIdentities` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::CounterForMembership` (r:1 w:1)
	/// Proof: `Membership::CounterForMembership` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Membership::MembershipsExpireOn` (r:1 w:1)
	/// Proof: `Membership::MembershipsExpireOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SmithMembers::Smiths` (r:3 w:3)
	/// Proof: `SmithMembers::Smiths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::Members` (r:1 w:1)
	/// Proof: `AuthorityMembers::Members` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OnlineAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OnlineAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OutgoingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OutgoingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::IncomingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::IncomingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:4 w:4)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityIndexOf` (r:0 w:1)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Session::KeyOwner` (r:0 w:4)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_remove_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1990`
		//  Estimated: `12880`
		// Minimum execution time: 110_128_000 picoseconds.
		Weight::from_parts(113_248_000, 0)
			.saturating_add(Weight::from_parts(0, 12880))
			.saturating_add(T::DbWeight::get().reads(21))
			.saturating_add(T::DbWeight::get().writes(26))
	}
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::CounterForMembership` (r:1 w:1)
	/// Proof: `Membership::CounterForMembership` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Membership::MembershipsExpireOn` (r:1 w:1)
	/// Proof: `Membership::MembershipsExpireOn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:2 w:2)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `UniversalDividend::CurrentUdIndex` (r:1 w:0)
	/// Proof: `UniversalDividend::CurrentUdIndex` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `SmithMembers::Smiths` (r:3 w:3)
	/// Proof: `SmithMembers::Smiths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::Members` (r:1 w:1)
	/// Proof: `AuthorityMembers::Members` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OnlineAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OnlineAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::OutgoingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::OutgoingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorityMembers::IncomingAuthorities` (r:1 w:1)
	/// Proof: `AuthorityMembers::IncomingAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:1)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::StorageIdtyCertMeta` (r:4 w:4)
	/// Proof: `Certification::StorageIdtyCertMeta` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parameters::ParametersStorage` (r:1 w:0)
	/// Proof: `Parameters::ParametersStorage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Session::KeyOwner` (r:0 w:4)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn do_remove_identity_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1999`
		//  Estimated: `12889`
		// Minimum execution time: 106_877_000 picoseconds.
		Weight::from_parts(111_899_000, 0)
			.saturating_add(Weight::from_parts(0, 12889))
			.saturating_add(T::DbWeight::get().reads(22))
			.saturating_add(T::DbWeight::get().writes(25))
	}
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityChangeSchedule` (r:2 w:2)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn membership_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `6347`
		// Minimum execution time: 14_063_000 picoseconds.
		Weight::from_parts(14_485_000, 0)
			.saturating_add(Weight::from_parts(0, 6347))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::IdentityChangeSchedule` (r:1 w:0)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn prune_identities_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `157`
		//  Estimated: `3622`
		// Minimum execution time: 2_535_000 picoseconds.
		Weight::from_parts(2_722_000, 0)
			.saturating_add(Weight::from_parts(0, 3622))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Identity::IdentityChangeSchedule` (r:1 w:1)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:0)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn prune_identities_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327`
		//  Estimated: `3792`
		// Minimum execution time: 5_830_000 picoseconds.
		Weight::from_parts(6_226_000, 0)
			.saturating_add(Weight::from_parts(0, 3792))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityChangeSchedule` (r:1 w:1)
	/// Proof: `Identity::IdentityChangeSchedule` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::Identities` (r:1 w:1)
	/// Proof: `Identity::Identities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::CounterForIdentities` (r:1 w:1)
	/// Proof: `Identity::CounterForIdentities` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Membership::Membership` (r:1 w:1)
	/// Proof: `Membership::Membership` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Certification::CertsByReceiver` (r:1 w:0)
	/// Proof: `Certification::CertsByReceiver` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Identity::IdentityIndexOf` (r:0 w:1)
	/// Proof: `Identity::IdentityIndexOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Quota::IdtyQuota` (r:0 w:1)
	/// Proof: `Quota::IdtyQuota` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn prune_identities_err() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `851`
		//  Estimated: `4316`
		// Minimum execution time: 29_919_000 picoseconds.
		Weight::from_parts(30_964_000, 0)
			.saturating_add(Weight::from_parts(0, 4316))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
