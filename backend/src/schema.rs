//! Iphone queue database schema.
use crate::queue::Queue;
use exonum::crypto::{Hash, PublicKey};
use exonum_merkledb::{IndexAccess, ObjectHash, ProofListIndex, ProofMapIndex};
use std::cmp::Ordering;

/// Pipe types table name
pub const Queues_TYPES_TABLE: &str = "queue_constructor.queue";
/// Pipe type history table name
pub const Queue_HISTORY_TABLE: &str = "queue_constructor.queue.history";

/// Database schema.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
where
    T: IndexAccess,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    /// Returns `ProofMapIndex` with pipe types.
    pub fn queues(&self) -> ProofMapIndex<T, PublicKey, Queue> {
        ProofMapIndex::new(Queues_TYPES_TABLE, self.view.clone())
    }

    /// Returns history of the pipe type with the given public key.
    pub fn queues_history(&self, public_key: &PublicKey) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(Queue_HISTORY_TABLE, public_key, self.view.clone())
    }

    /// Returns pipe type for the given public key.
    pub fn queue(&self, pub_key: &PublicKey) -> Option<Queue> {
        self.queues().get(pub_key)
    }

    /// Returns the state hash of service.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.queues().object_hash()]
    }

    /// Create new participant and append first record to its history.
    pub fn add_queue(
        &mut self,
        key: &PublicKey,
        name: string
    ) {
        let created_queue = {
          
            Queue::new(
                key,
                name
            )
        };
        self.queus().put(key, created_queue);
    }
   
}