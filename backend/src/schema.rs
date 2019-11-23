//! Iphone queue database schema.
use crate::queue::Queue;
use crate::queue_attributes::AttributesInQueue;
use crate::user::User;
use exonum::crypto::{Hash, PublicKey};
use exonum_merkledb::{IndexAccess, ObjectHash, ProofListIndex, ProofMapIndex};
use std::cmp::Ordering;

/// Pipe types table name
pub const Queues_TYPES_TABLE: &str = "queue_constructor.queue";
/// Pipe type history table name
pub const Queue_HISTORY_TABLE: &str = "queue_constructor.queue.history";
///
pub const Queues_attribues_TYPES_TABLE: &str = "queue_attributes_constructor.queue";
///
pub const Users_TABLE: &str = "queue_users";
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
    ///
    pub fn attributes_in_queues(&self) -> ProofMapIndex<T, PublicKey, AttributesInQueue> {
        ProofMapIndex::new(Queues_attribues_TYPES_TABLE, self.view.clone())
    }
    ///
    pub fn users(&self) -> ProofMapIndex<T, PublicKey, User> {
        ProofMapIndex::new(Users_TABLE, self.view.clone())     
    }
    ///
    pub fn user(&self, pub_key: &PublicKey) -> Option<User> {
        self.users().get(pub_key)     
    }
     /// Create new Queue and append first record to its history.
    pub fn add_queue(
        &mut self,
        key: &PublicKey,
        name: &String
    ) {
        let created_queue = {
          
            Queue::new(
                key,
                &name
            )
        };
        self.queues().put(key, created_queue);
    }
    ///method for adding attributes to queu
    pub fn add_attributes_to_queue (
        &mut self,
        key: &PublicKey,
        QueueKey: PublicKey,
        name: String,
        typeAttribute:String,
        order:u64,
        sortable:bool,
        obligatory:bool,
        priorityInOrder:bool,
        coefficient:u64,
    )  {
        let attributes_in_queue = {
            AttributesInQueue:: new(
                key,
                QueueKey,
                &name,
                typeAttribute,
                order,
                sortable,
                obligatory,
                priorityInOrder,
                coefficient,
            )
        };
        self.attributes_in_queues().put(key, attributes_in_queue);
    }
}
