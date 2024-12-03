use std::future::{Future, IntoFuture};

use alloy::{
    network::Network,
    primitives::{Address, StorageKey, StorageValue},
    providers::Provider,
    transports::{RpcError, Transport, TransportErrorKind},
};

/// Provides arbitrary state from an EVM chain
pub trait BlockchainDataProvider {
    type Error;

    /// Returns the storage value for the given address at the given storage slot
    fn storage_slot_at(
        &self,
        address: Address,
        slot: StorageKey,
    ) -> impl Future<Output = Result<StorageValue, Self::Error>>;
}

/// Newtype wrapper around Alloy's [`Provider`] trait
///
/// Sadly necessary due to the requirement to implement [`BlockchainDataProvider`]
pub struct AlloyProvider<T: Transport + Clone, N: Network>(pub Box<dyn Provider<T, N>>);

impl<T, N> BlockchainDataProvider for AlloyProvider<T, N>
where
    T: Transport + Clone,
    N: Network,
{
    type Error = RpcError<TransportErrorKind>;

    fn storage_slot_at(
        &self,
        address: Address,
        slot: StorageKey,
    ) -> impl Future<Output = Result<StorageValue, Self::Error>> {
        self.0.get_storage_at(address, slot.into()).into_future()
    }
}
