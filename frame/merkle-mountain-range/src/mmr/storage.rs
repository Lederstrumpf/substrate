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

//! A MMR storage implementations.

use codec::Encode;
use frame_support::log;
use log::info;
use mmr_lib::helper;
use sp_io::offchain_index;
use sp_runtime::traits::Saturating;
use sp_std::iter::Peekable;
#[cfg(not(feature = "std"))]
use sp_std::prelude::*;

use crate::{
	mmr::{utils::NodesUtils, Node, NodeOf},
	primitives::{self, NodeIndex},
	Config, Nodes, NumberOfLeaves, Pallet,
};

/// A marker type for runtime-specific storage implementation.
///
/// Allows appending new items to the MMR and proof verification.
/// MMR nodes are appended to two different storages:
/// 1. We add nodes (leaves) hashes to the on-chain storage (see [crate::Nodes]).
/// 2. We add full leaves (and all inner nodes as well) into the `IndexingAPI` during block
///    processing, so the values end up in the Offchain DB if indexing is enabled.
pub struct RuntimeStorage;

/// A marker type for offchain-specific storage implementation.
///
/// Allows proof generation and verification, but does not support appending new items.
/// MMR nodes are assumed to be stored in the Off-Chain DB. Note this storage type
/// DOES NOT support adding new items to the MMR.
pub struct OffchainStorage;

/// A storage layer for MMR.
///
/// There are two different implementations depending on the use case.
/// See docs for [RuntimeStorage] and [OffchainStorage].
pub struct Storage<StorageType, T, I, L>(sp_std::marker::PhantomData<(StorageType, T, I, L)>);

impl<StorageType, T, I, L> Default for Storage<StorageType, T, I, L> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<StorageType, T, I, L> Storage<StorageType, T, I, L>
where
	T: Config<I>,
	I: 'static,
	L: primitives::FullLeaf + codec::Decode,
{
	fn parent_hash_of_ancestor_that_added_node(
		pos: NodeIndex,
	) -> <T as frame_system::Config>::Hash {
		let leaves_count: <T as frame_system::Config>::BlockNumber =
			u32::try_from(NumberOfLeaves::<T, I>::get())
				.expect("leaf-idx < block-num; qed")
				.into();
		let ancestor_leaf_idx = u32::try_from(NodesUtils::leaf_index_that_added_node(pos))
			.expect("leaf-idx < block-num; qed")
			.into();
		// leaves are zero-indexed and were added one per block since pallet activation,
		// while block numbers are one-indexed, so block number that added `leaf_idx` is:
		// `block_num = block_num_when_pallet_activated + leaf_idx + 1`
		// `block_num = (current_block_num - leaves_count) + leaf_idx + 1`
		// `parent_block_num = current_block_num - leaves_count + leaf_idx`.
		let parent_block_num: <T as frame_system::Config>::BlockNumber =
			<frame_system::Pallet<T>>::block_number()
				.saturating_sub(leaves_count)
				.saturating_add(ancestor_leaf_idx);

		// TODO: I think this only holds recent history, so old block hashes might not be here.
		let parent_hash = <frame_system::Pallet<T>>::block_hash(parent_block_num);
		info!(
			target: "runtime::mmr",
			"🥩: parent of ancestor that added {}: leaf idx {:?}, block-num {:?} (block offset {:?}) hash {:?}",
			pos, ancestor_leaf_idx, parent_block_num,
			<frame_system::Pallet<T>>::block_number().saturating_sub(leaves_count),
			parent_hash
		);
		parent_hash
	}
}

impl<T, I, L> mmr_lib::MMRStore<NodeOf<T, I, L>> for Storage<OffchainStorage, T, I, L>
where
	T: Config<I>,
	I: 'static,
	L: primitives::FullLeaf + codec::Decode,
{
	fn get_elem(&self, pos: NodeIndex) -> mmr_lib::Result<Option<NodeOf<T, I, L>>> {
		// Get the parent hash of the ancestor block that added node at index `pos`.
		// Use the hash as extra identifier to differentiate between various `pos` entries
		// in offchain DB coming from various chain forks.
		let parent_hash_of_ancestor = Self::parent_hash_of_ancestor_that_added_node(pos);
		let key = Pallet::<T, I>::offchain_key(parent_hash_of_ancestor, pos);
		info!(
			target: "runtime::mmr",
			"🥩: get elem {}: key {:?}",
			pos, key
		);
		// Retrieve the element from Off-chain DB.
		Ok(sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
			.and_then(|v| codec::Decode::decode(&mut &*v).ok()))
	}

	fn append(&mut self, _: NodeIndex, _: Vec<NodeOf<T, I, L>>) -> mmr_lib::Result<()> {
		panic!("MMR must not be altered in the off-chain context.")
	}
}

impl<T, I, L> mmr_lib::MMRStore<NodeOf<T, I, L>> for Storage<RuntimeStorage, T, I, L>
where
	T: Config<I>,
	I: 'static,
	L: primitives::FullLeaf,
{
	fn get_elem(&self, pos: NodeIndex) -> mmr_lib::Result<Option<NodeOf<T, I, L>>> {
		Ok(<Nodes<T, I>>::get(pos).map(Node::Hash))
	}

	fn append(&mut self, pos: NodeIndex, elems: Vec<NodeOf<T, I, L>>) -> mmr_lib::Result<()> {
		if elems.is_empty() {
			return Ok(())
		}

		sp_std::if_std! {
			frame_support::log::info!("elems: {:?}", elems.iter().map(|elem| elem.hash()).collect::<Vec<_>>());
		}

		let leaves = NumberOfLeaves::<T, I>::get();
		let size = NodesUtils::new(leaves).size();

		info!(
			target: "runtime::mmr",
			"🥩: append elem {}: leaves {} size {}",
			pos, leaves, size
		);

		if pos != size {
			return Err(mmr_lib::Error::InconsistentStore)
		}

		let new_size = size + elems.len() as NodeIndex;

		// A sorted (ascending) iterator over peak indices to prune and persist.
		let (peaks_to_prune, mut peaks_to_store) = peaks_to_prune_and_store(size, new_size);

		// Now we are going to iterate over elements to insert
		// and keep track of the current `node_index` and `leaf_index`.
		let mut leaf_index = leaves;
		let mut node_index = size;

		// Use parent hash of block adding new nodes (this block) as extra identifier
		// in offchain DB to avoid DB collisions and overwrites in case of forks.
		let parent_hash = <frame_system::Pallet<T>>::parent_hash();
		let block_number = <frame_system::Pallet<T>>::block_number();
		for elem in elems {
			let key = Pallet::<T, I>::offchain_key(parent_hash, node_index);
			info!(
				target: "runtime::mmr",
				"🥩: offchain set: block-num {:?}, parent_hash {:?} node-idx {} key {:?}",
				block_number, parent_hash, node_index, key
			);
			// Indexing API is used to store the full node content (both leaf and inner).
			elem.using_encoded(|elem| offchain_index::set(&key, elem));

			// On-chain we are going to only store new peaks.
			if peaks_to_store.next_if_eq(&node_index).is_some() {
				<Nodes<T, I>>::insert(node_index, elem.hash());
			}

			// Increase the indices.
			if let Node::Data(..) = elem {
				leaf_index += 1;
			}
			node_index += 1;
		}

		// Update current number of leaves.
		NumberOfLeaves::<T, I>::put(leaf_index);

		// And remove all remaining items from `peaks_before` collection.
		for pos in peaks_to_prune {
			<Nodes<T, I>>::remove(pos);
		}

		Ok(())
	}
}

fn peaks_to_prune_and_store(
	old_size: NodeIndex,
	new_size: NodeIndex,
) -> (impl Iterator<Item = NodeIndex>, Peekable<impl Iterator<Item = NodeIndex>>) {
	// A sorted (ascending) collection of peak indices before and after insertion.
	// both collections may share a common prefix.
	let peaks_before = if old_size == 0 { vec![] } else { helper::get_peaks(old_size) };
	let peaks_after = helper::get_peaks(new_size);
	sp_std::if_std! {
		frame_support::log::trace!("peaks_before: {:?}", peaks_before);
		frame_support::log::trace!("peaks_after: {:?}", peaks_after);
	}
	let mut peaks_before = peaks_before.into_iter().peekable();
	let mut peaks_after = peaks_after.into_iter().peekable();

	// Consume a common prefix between `peaks_before` and `peaks_after`,
	// since that's something we will not be touching anyway.
	while peaks_before.peek() == peaks_after.peek() {
		peaks_before.next();
		peaks_after.next();
	}

	// what's left in both collections is:
	// 1. Old peaks to remove from storage
	// 2. New peaks to persist in storage
	(peaks_before, peaks_after)
}
