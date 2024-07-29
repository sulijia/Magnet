
//! Autogenerated weights for `pallet_order`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sulijia-pc`, CPU: `AMD Ryzen 7 5800U with Radeon Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/parachain-magnet-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_order
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/order/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;
use frame_support::weights::constants::RocksDbWeight;

pub trait WeightInfo {
	fn set_slot_width() -> Weight;
	fn set_price_limit() -> Weight;
	fn set_gas_threshold() -> Weight;
	fn create_order() -> Weight;
}
/// Weight functions for `pallet_order`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `OrderPallet::SlotWidth` (r:0 w:1)
	/// Proof: `OrderPallet::SlotWidth` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_slot_width() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_505_000 picoseconds.
		Weight::from_parts(2_739_117, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 96
			.saturating_add(Weight::from_parts(153, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::PriceLimit` (r:0 w:1)
	/// Proof: `OrderPallet::PriceLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_price_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_515_000 picoseconds.
		Weight::from_parts(2_786_295, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 470
			.saturating_add(Weight::from_parts(180, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::GasThreshold` (r:0 w:1)
	/// Proof: `OrderPallet::GasThreshold` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_gas_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_505_000 picoseconds.
		Weight::from_parts(2_766_066, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::SequenceNumber` (r:1 w:1)
	/// Proof: `OrderPallet::SequenceNumber` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::OrderMap` (r:1 w:1)
	/// Proof: `OrderPallet::OrderMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Aura::Authorities` (r:1 w:0)
	/// Proof: `Aura::Authorities` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::SlotWidth` (r:1 w:0)
	/// Proof: `OrderPallet::SlotWidth` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::Block2Sequence` (r:0 w:1)
	/// Proof: `OrderPallet::Block2Sequence` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `339`
		//  Estimated: `3201489`
		// Minimum execution time: 23_434_000 picoseconds.
		Weight::from_parts(25_538_352, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			// Standard Error: 11_114
			.saturating_add(Weight::from_parts(21_074, 0))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

impl WeightInfo for () {
	/// Storage: `OrderPallet::SlotWidth` (r:0 w:1)
	/// Proof: `OrderPallet::SlotWidth` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_slot_width() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_505_000 picoseconds.
		Weight::from_parts(2_739_117, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 96
			.saturating_add(Weight::from_parts(153, 0))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::PriceLimit` (r:0 w:1)
	/// Proof: `OrderPallet::PriceLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_price_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_515_000 picoseconds.
		Weight::from_parts(2_786_295, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 470
			.saturating_add(Weight::from_parts(180, 0))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::GasThreshold` (r:0 w:1)
	/// Proof: `OrderPallet::GasThreshold` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_gas_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_505_000 picoseconds.
		Weight::from_parts(2_766_066, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `OrderPallet::SequenceNumber` (r:1 w:1)
	/// Proof: `OrderPallet::SequenceNumber` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::OrderMap` (r:1 w:1)
	/// Proof: `OrderPallet::OrderMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Aura::Authorities` (r:1 w:0)
	/// Proof: `Aura::Authorities` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::SlotWidth` (r:1 w:0)
	/// Proof: `OrderPallet::SlotWidth` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OrderPallet::Block2Sequence` (r:0 w:1)
	/// Proof: `OrderPallet::Block2Sequence` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `339`
		//  Estimated: `3201489`
		// Minimum execution time: 23_434_000 picoseconds.
		Weight::from_parts(25_538_352, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			// Standard Error: 11_114
			.saturating_add(Weight::from_parts(21_074, 0))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}