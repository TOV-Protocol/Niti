
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-19 (Y/M/D)
//! HOSTNAME: `benjamin-xps139380`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//!
//! SHORT-NAME: `block`, LONG-NAME: `BlockExecution`, RUNTIME: `Development`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `runtime/common/src/weights/`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1.0`, WEIGHT-ADD: `0`

// Executed Command:
//   target/release/duniter
//   benchmark
//   overhead
//   --chain=gdev-benchmark
//   --execution=wasm
//   --wasm-execution=compiled
//   --weight-path=runtime/common/src/weights/
//   --warmup=10

use sp_core::parameter_types;
use sp_weights::{constants::WEIGHT_PER_NANOS, Weight};

parameter_types! {
	/// Time to execute an empty block.
	/// Calculated by multiplying the *Average* with `1.0` and adding `0`.
	///
	/// Stats nanoseconds:
	///   Min, Max: 698_202, 838_988
	///   Average:  709_232
	///   Median:   704_251
	///   Std-Dev:  15664.54
	///
	/// Percentiles nanoseconds:
	///   99th: 738_574
	///   95th: 729_596
	///   75th: 709_823
	pub const BlockExecutionWeight: Weight = WEIGHT_PER_NANOS.saturating_mul(709_232);
}

#[cfg(test)]
mod test_weights {
	use sp_weights::constants;

	/// Checks that the weight exists and is sane.
	// NOTE: If this test fails but you are sure that the generated values are fine,
	// you can delete it.
	#[test]
	fn sane() {
		let w = super::BlockExecutionWeight::get();

		// At least 100 µs.
		assert!(
			w.ref_time() >= 100u64 * constants::WEIGHT_PER_MICROS.ref_time(),
			"Weight should be at least 100 µs."
		);
		// At most 50 ms.
		assert!(
			w.ref_time() <= 50u64 * constants::WEIGHT_PER_MILLIS.ref_time(),
			"Weight should be at most 50 ms."
		);
	}
}
