
// This file is part of Substrate.

// Copyright (C) 2020-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! <!-- markdown-link-check-disable -->
//! # Offchain Worker Example Pallet
//!
//! The Offchain Worker Example: A simple pallet demonstrating
//! concepts, APIs and structures common to most offchain workers.
//!
//! Run `cargo doc --package pallet-example-offchain-worker --open` to view this module's
//! documentation.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! **This pallet serves as an example showcasing Substrate off-chain worker and is not meant to
//! be used in production.**
//!
//! ## Overview
//!
//! In this example we are going to build a very simplistic, naive and definitely NOT
//! production-ready oracle for BTC/USD price.
//! Offchain Worker (OCW) will be triggered after every block, fetch the current price
//! and prepare either signed or unsigned transaction to feed the result back on chain.
//! The on-chain logic will simply aggregate the results and store last `64` values to compute
//! the average price.
//! Additional logic in OCW is put in place to prevent spamming the network with both signed
//! and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only
//! one unsigned transaction floating in the network.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	self as system,
};

use sp_runtime::{
	offchain::{
		http,
	},
};
use sp_std::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn other_rpc)]
	pub(super) type OtherRpc<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain Worker entry point.
		///
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// succesfuly imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		/// You can use `Local Storage` API to coordinate runs of the worker.
		fn offchain_worker(block_number: T::BlockNumber) {
			Self::fetch_price();
		}
	}


	impl<T: Config> Pallet<T> {
		/// Fetch current price and return the result in cents.
		fn fetch_price() -> Result<(), http::Error> {
	
			// TODO: get this from runtime storage
			let url = "https://moonbase-alpha.public.blastapi.io";
	
			// TODO: generate the json 
			let body = b"{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_chain\"}";
	
			let pending =http::Request::default()
				.method(http::Method::Post)
				.url(&url)
				.body(sp_std::vec![body])
				.add_header("Content-Type", "application/json")
				.send()
				.unwrap();
	
			// wait
			let mut response = pending.wait().unwrap();
			log::info!("the response was{:?}", response);
	
			// Parse response... etc
			// let body = response.body().collect::<Vec<u8>>();	
			// // Create a str slice from the body.
			// let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
			// 	log::warn!("No UTF8 body");
			// 	http::Error::Unknown
			// })?;
	
			// log::info!("the body was{:?}", response);


			Ok(())
		}

	}

}

