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


//! Autogenerated weights for runtime_parachains::assigner_on_demand
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-09-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("starlight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// runtime_parachains::assigner_on_demand
// --extrinsic
// *
// --chain=starlight-dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/starlight_weights/runtime_parachains::assigner_on_demand.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for runtime_parachains::assigner_on_demand using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::assigner_on_demand::WeightInfo for SubstrateWeight<T> {
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `OnDemandAssignmentProvider::Revenue` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::Revenue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::ParaIdAffinity` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::ParaIdAffinity` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[1, 9999]`.
	fn place_order_keep_alive(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237 + s * (8 ±0)`
		//  Estimated: `3700 + s * (8 ±0)`
		// Minimum execution time: 46_661_000 picoseconds.
		Weight::from_parts(52_726_646, 3700)
			// Standard Error: 117
			.saturating_add(Weight::from_parts(20_346, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(s.into()))
	}
	/// Storage: `OnDemandAssignmentProvider::QueueStatus` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::QueueStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `OnDemandAssignmentProvider::Revenue` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::Revenue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::ParaIdAffinity` (r:1 w:0)
	/// Proof: `OnDemandAssignmentProvider::ParaIdAffinity` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `OnDemandAssignmentProvider::FreeEntries` (r:1 w:1)
	/// Proof: `OnDemandAssignmentProvider::FreeEntries` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[1, 9999]`.
	fn place_order_allow_death(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237 + s * (8 ±0)`
		//  Estimated: `3700 + s * (8 ±0)`
		// Minimum execution time: 46_599_000 picoseconds.
		Weight::from_parts(52_157_164, 3700)
			// Standard Error: 122
			.saturating_add(Weight::from_parts(20_538, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 8).saturating_mul(s.into()))
	}
}