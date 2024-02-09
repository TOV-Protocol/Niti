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

//! Autogenerated weights for `pallet_collective`
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

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `TechnicalCommittee::Members` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:20 w:20)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 20]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (672 ±0) + p * (3191 ±0)`
		//  Estimated: `10019 + m * (416 ±5) + p * (4183 ±27)`
		// Minimum execution time: 11_299_000 picoseconds.
		Weight::from_parts(11_509_000, 0)
			.saturating_add(Weight::from_parts(0, 10019))
			// Standard Error: 8_549
			.saturating_add(Weight::from_parts(644_119, 0).saturating_mul(m.into()))
			// Standard Error: 42_263
			.saturating_add(Weight::from_parts(5_653_926, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 416).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4183).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `1518 + m * (32 ±0)`
		// Minimum execution time: 9_634_000 picoseconds.
		Weight::from_parts(9_182_276, 0)
			.saturating_add(Weight::from_parts(0, 1518))
			// Standard Error: 27
			.saturating_add(Weight::from_parts(1_193, 0).saturating_mul(b.into()))
			// Standard Error: 287
			.saturating_add(Weight::from_parts(12_966, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `3498 + m * (32 ±0)`
		// Minimum execution time: 11_648_000 picoseconds.
		Weight::from_parts(11_249_880, 0)
			.saturating_add(Weight::from_parts(0, 3498))
			// Standard Error: 36
			.saturating_add(Weight::from_parts(1_155, 0).saturating_mul(b.into()))
			// Standard Error: 374
			.saturating_add(Weight::from_parts(19_049, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalCount` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 20]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `24 + m * (32 ±0) + p * (55 ±0)`
		//  Estimated: `3461 + m * (32 ±0) + p * (54 ±0)`
		// Minimum execution time: 16_530_000 picoseconds.
		Weight::from_parts(15_662_728, 0)
			.saturating_add(Weight::from_parts(0, 3461))
			// Standard Error: 63
			.saturating_add(Weight::from_parts(2_178, 0).saturating_mul(b.into()))
			// Standard Error: 661
			.saturating_add(Weight::from_parts(17_907, 0).saturating_mul(m.into()))
			// Standard Error: 3_307
			.saturating_add(Weight::from_parts(259_014, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 54).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `573 + m * (64 ±0)`
		//  Estimated: `4037 + m * (64 ±0)`
		// Minimum execution time: 15_171_000 picoseconds.
		Weight::from_parts(15_664_934, 0)
			.saturating_add(Weight::from_parts(0, 4037))
			// Standard Error: 548
			.saturating_add(Weight::from_parts(35_118, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117 + m * (64 ±0) + p * (55 ±0)`
		//  Estimated: `3591 + m * (64 ±0) + p * (55 ±0)`
		// Minimum execution time: 18_725_000 picoseconds.
		Weight::from_parts(17_295_145, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			// Standard Error: 611
			.saturating_add(Weight::from_parts(31_903, 0).saturating_mul(m.into()))
			// Standard Error: 3_017
			.saturating_add(Weight::from_parts(236_285, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `62 + b * (1 ±0) + m * (64 ±0) + p * (78 ±0)`
		//  Estimated: `3619 + b * (1 ±0) + m * (63 ±0) + p * (74 ±0)`
		// Minimum execution time: 27_086_000 picoseconds.
		Weight::from_parts(25_836_642, 0)
			.saturating_add(Weight::from_parts(0, 3619))
			// Standard Error: 83
			.saturating_add(Weight::from_parts(1_757, 0).saturating_mul(b.into()))
			// Standard Error: 885
			.saturating_add(Weight::from_parts(18_479, 0).saturating_mul(m.into()))
			// Standard Error: 4_366
			.saturating_add(Weight::from_parts(334_711, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 63).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + m * (64 ±0) + p * (55 ±0)`
		//  Estimated: `3611 + m * (64 ±0) + p * (55 ±0)`
		// Minimum execution time: 20_995_000 picoseconds.
		Weight::from_parts(18_820_652, 0)
			.saturating_add(Weight::from_parts(0, 3611))
			// Standard Error: 615
			.saturating_add(Weight::from_parts(38_431, 0).saturating_mul(m.into()))
			// Standard Error: 3_037
			.saturating_add(Weight::from_parts(252_565, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82 + b * (1 ±0) + m * (64 ±0) + p * (78 ±0)`
		//  Estimated: `3639 + b * (1 ±0) + m * (63 ±0) + p * (74 ±0)`
		// Minimum execution time: 28_333_000 picoseconds.
		Weight::from_parts(26_411_146, 0)
			.saturating_add(Weight::from_parts(0, 3639))
			// Standard Error: 88
			.saturating_add(Weight::from_parts(2_323, 0).saturating_mul(b.into()))
			// Standard Error: 935
			.saturating_add(Weight::from_parts(21_833, 0).saturating_mul(m.into()))
			// Standard Error: 4_613
			.saturating_add(Weight::from_parts(354_718, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 63).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 20]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189 + p * (32 ±0)`
		//  Estimated: `1674 + p * (32 ±0)`
		// Minimum execution time: 9_381_000 picoseconds.
		Weight::from_parts(10_496_418, 0)
			.saturating_add(Weight::from_parts(0, 1674))
			// Standard Error: 2_040
			.saturating_add(Weight::from_parts(164_732, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}
