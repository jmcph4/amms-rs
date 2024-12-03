use std::future::Future;

use alloy::primitives::{Address, StorageKey, StorageValue};

/// Provides arbitrary state from an EVM chain
pub trait BlockchainDataProvider {
    type Error;

    /// Returns the storage value for the given address at the given storage slot
    fn storage_slot_at(
        address: Address,
        slot: StorageKey,
    ) -> impl Future<Output = Result<StorageValue, Self::Error>> + Send;
}
