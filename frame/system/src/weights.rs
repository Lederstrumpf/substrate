// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/system/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for frame_system.
pub trait WeightInfo {
	fn remark(b: u32, ) -> Weight;
	fn remark_with_event(b: u32, ) -> Weight;
	fn set_heap_pages() -> Weight;
	fn set_storage(i: u32, ) -> Weight;
	fn kill_storage(i: u32, ) -> Weight;
	fn kill_prefix(p: u32, ) -> Weight;
	fn worst_case_write_no_transactional() -> Weight;
	fn worst_case_transactional_no_write(l: u32, ) -> Weight;
	fn worst_case_transactional_write(l: u32, ) -> Weight;
}

/// Weights for frame_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: crate::Config> WeightInfo for SubstrateWeight<T> {
	fn remark(_b: u32, ) -> Weight {
		(0 as Weight)
	}
	fn remark_with_event(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		(2_952_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn set_storage(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((417_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_storage(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((312_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_prefix(p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((650_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn worst_case_write_no_transactional() -> Weight {
		(529_094_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1000 as Weight))
	}
	fn worst_case_transactional_no_write(l: u32, ) -> Weight {
		(715_000 as Weight)
			// Standard Error: 0
			.saturating_add((49_000 as Weight).saturating_mul(l as Weight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn worst_case_transactional_write(l: u32, ) -> Weight {
		(480_023_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((146_163_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().writes(1000 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn remark(_b: u32, ) -> Weight {
		(0 as Weight)
	}
	fn remark_with_event(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		(2_952_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn set_storage(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((417_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_storage(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((312_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_prefix(p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((650_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn worst_case_write_no_transactional() -> Weight {
		(529_094_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1000 as Weight))
	}
	fn worst_case_transactional_no_write(l: u32, ) -> Weight {
		(715_000 as Weight)
			// Standard Error: 0
			.saturating_add((49_000 as Weight).saturating_mul(l as Weight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn worst_case_transactional_write(l: u32, ) -> Weight {
		(480_023_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((146_163_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().writes(1000 as Weight))
	}
}
