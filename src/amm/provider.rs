use std::future::Future;

use alloy::primitives::{Address, U256};

/// EVM word
pub type Word = U256;

/// A **storage slot** in the EVM storage tree (not a consensus slot!)
pub type Slot = u64;

/// Provides arbitrary state from an EVM chain
pub trait BlockchainDataProvider {
    type Error;

    /// Returns the storage value for the given address at the given **storage slot**
    fn storage_slot_at(
        address: Address,
        slot: Slot,
    ) -> impl Future<Output = Result<Word, Self::Error>> + Send;
}
