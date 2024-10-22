// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_external_validators
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-10-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dancelight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_external_validators
// --extrinsic
// *
// --chain=dancelight-dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-pallet-template.hbs
// --json-file
// raw.json
// --output
// tmp/dancelight_weights/pallet_external_validators.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_external_validators.
pub trait WeightInfo {
	fn skip_external_validators() -> Weight;
	fn add_whitelisted(b: u32, ) -> Weight;
	fn remove_whitelisted(b: u32, ) -> Weight;
	fn force_no_eras() -> Weight;
	fn force_new_era() -> Weight;
	fn force_new_era_always() -> Weight;
	fn new_session(r: u32, ) -> Weight;
}

/// Weights for pallet_external_validators using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ExternalValidators::SkipExternalValidators` (r:0 w:1)
	/// Proof: `ExternalValidators::SkipExternalValidators` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn skip_external_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_391_000 picoseconds.
		Weight::from_parts(1_484_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:1)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 99]`.
	fn add_whitelisted(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `845 + b * (36 ±0)`
		//  Estimated: `4687 + b * (37 ±0)`
		// Minimum execution time: 12_829_000 picoseconds.
		Weight::from_parts(17_541_907, 4687)
			// Standard Error: 1_560
			.saturating_add(Weight::from_parts(62_143, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
	}
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:1)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn remove_whitelisted(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 7_269_000 picoseconds.
		Weight::from_parts(9_100_286, 4687)
			// Standard Error: 626
			.saturating_add(Weight::from_parts(35_303, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_no_eras() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_578_000 picoseconds.
		Weight::from_parts(4_924_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_new_era() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_727_000 picoseconds.
		Weight::from_parts(4_990_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_new_era_always() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_648_000 picoseconds.
		Weight::from_parts(4_863_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:1 w:0)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::EraSessionStart` (r:1 w:1)
	/// Proof: `ExternalValidators::EraSessionStart` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::ActiveEra` (r:1 w:1)
	/// Proof: `ExternalValidators::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::SkipExternalValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::SkipExternalValidators` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::ExternalValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::ExternalValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn new_session(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + r * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 8_587_000 picoseconds.
		Weight::from_parts(10_453_582, 4687)
			// Standard Error: 555
			.saturating_add(Weight::from_parts(27_159, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `ExternalValidators::SkipExternalValidators` (r:0 w:1)
	/// Proof: `ExternalValidators::SkipExternalValidators` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn skip_external_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_391_000 picoseconds.
		Weight::from_parts(1_484_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:1)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 99]`.
	fn add_whitelisted(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `845 + b * (36 ±0)`
		//  Estimated: `4687 + b * (37 ±0)`
		// Minimum execution time: 12_829_000 picoseconds.
		Weight::from_parts(17_541_907, 4687)
			// Standard Error: 1_560
			.saturating_add(Weight::from_parts(62_143, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
	}
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:1)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn remove_whitelisted(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 7_269_000 picoseconds.
		Weight::from_parts(9_100_286, 4687)
			// Standard Error: 626
			.saturating_add(Weight::from_parts(35_303, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_no_eras() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_578_000 picoseconds.
		Weight::from_parts(4_924_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_new_era() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_727_000 picoseconds.
		Weight::from_parts(4_990_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:0 w:1)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_new_era_always() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_648_000 picoseconds.
		Weight::from_parts(4_863_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ForceEra` (r:1 w:0)
	/// Proof: `ExternalValidators::ForceEra` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::EraSessionStart` (r:1 w:1)
	/// Proof: `ExternalValidators::EraSessionStart` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::ActiveEra` (r:1 w:1)
	/// Proof: `ExternalValidators::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::WhitelistedValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::WhitelistedValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::SkipExternalValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::SkipExternalValidators` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidators::ExternalValidators` (r:1 w:0)
	/// Proof: `ExternalValidators::ExternalValidators` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn new_session(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + r * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 8_587_000 picoseconds.
		Weight::from_parts(10_453_582, 4687)
			// Standard Error: 555
			.saturating_add(Weight::from_parts(27_159, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
