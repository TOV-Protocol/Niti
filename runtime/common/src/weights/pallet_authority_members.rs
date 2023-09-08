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

//! Autogenerated weights for `pallet_authority_members`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benjamin-xps139380`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("gdev-benchmark"), DB CACHE: 1024

// Executed Command:
// ./target/release/duniter
// benchmark
// pallet
// --chain
// gdev-benchmark
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet-authority-members
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output=runtime/common/src/weights/
// --header
// file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_authority_members`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_authority_members::WeightInfo for WeightInfo<T> {
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: SmithMembership Membership (r:1 w:0)
	// Storage: AuthorityMembers Members (r:1 w:0)
	// Storage: AuthorityMembers OutgoingAuthorities (r:1 w:1)
	// Storage: AuthorityMembers IncomingAuthorities (r:1 w:0)
	// Storage: AuthorityMembers OnlineAuthorities (r:1 w:0)
	// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	fn go_offline() -> Weight {
		// Minimum execution time: 106_597 nanoseconds.
		Weight::from_ref_time(109_880_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: SmithMembership Membership (r:1 w:0)
	// Storage: AuthorityMembers BlackList (r:1 w:0)
	// Storage: AuthorityMembers Members (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: AuthorityMembers IncomingAuthorities (r:1 w:1)
	// Storage: AuthorityMembers OutgoingAuthorities (r:1 w:0)
	// Storage: AuthorityMembers OnlineAuthorities (r:1 w:0)
	// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	fn go_online() -> Weight {
		// Minimum execution time: 132_009 nanoseconds.
		Weight::from_ref_time(204_473_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: SmithMembership Membership (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: Session KeyOwner (r:4 w:0)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: AuthorityMembers Members (r:1 w:1)
	// Storage: AuthorityMembers MustRotateKeysBefore (r:1 w:1)
	fn set_session_keys() -> Weight {
		// Minimum execution time: 161_156 nanoseconds.
		Weight::from_ref_time(182_210_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AuthorityMembers Members (r:1 w:1)
	// Storage: AuthorityMembers OnlineAuthorities (r:1 w:1)
	// Storage: AuthorityMembers OutgoingAuthorities (r:1 w:1)
	// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	// Storage: AuthorityMembers IncomingAuthorities (r:1 w:1)
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: SmithMembership Membership (r:1 w:1)
	// Storage: SmithMembership CounterForMembership (r:1 w:1)
	// Storage: Session KeyOwner (r:0 w:4)
	fn remove_member() -> Weight {
		// Minimum execution time: 225_027 nanoseconds.
		Weight::from_ref_time(243_550_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
	// Storage: AuthorityMembers BlackList (r:1 w:1)
	fn remove_member_from_blacklist() -> Weight {
		// Minimum execution time: 60_023 nanoseconds.
		Weight::from_ref_time(60_615_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}