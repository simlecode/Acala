// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_idle_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ea4c8813bb44`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_idle_scheduler.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_idle_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: IdleScheduler PreviousRelayBlockNumber (r:0 w:1)
	fn on_initialize() -> Weight {
		// Minimum execution time: 3_586 nanoseconds.
		Weight::from_parts(3_753_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: IdleScheduler PreviousRelayBlockNumber (r:1 w:0)
	fn on_idle_base() -> Weight {
		// Minimum execution time: 5_240 nanoseconds.
		Weight::from_parts(5_421_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: IdleScheduler Tasks (r:0 w:1)
	fn clear_tasks() -> Weight {
		// Minimum execution time: 9_561 nanoseconds.
		Weight::from_parts(9_793_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	fn schedule_task() -> Weight {
		// Minimum execution time: 16_582 nanoseconds.
		Weight::from_parts(16_940_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
