//! Iphone queue database schema.
use crate::queue::Queue;
use crate::queue_attributes::AttributesInQueue;
use crate::user::User;
use crate::profile::Profile;
use crate::profile_attribute_value::ProfileAttributeValue;
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
///
pub const Profiles_TABLE: &str = "queue_users";
///
pub const Profiles_attributes_TABLE: &str = "queue_users_atrs";
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
    pub fn queues_attr(&self, public_key: &PublicKey) -> ProofListIndex<T, AttributesInQueue> {
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
    ///
    pub fn profiles(&self) -> ProofMapIndex<T, PublicKey, Profile> {
        ProofMapIndex::new(Profiles_TABLE, self.view.clone())     
    }
    ///
    pub fn profile(&self, pub_key: &PublicKey) -> Option<Profile> {
        self.profiles().get(pub_key)  
    }
    ///
    pub fn profiles_attributes(&self) -> ProofMapIndex<T, PublicKey, ProfileAttributeValue> {
        ProofMapIndex::new(Profiles_attributes_TABLE, self.view.clone())     
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
        name: &String,
        typeAttribute:&String,
        order:&String,
        sortable:u64,
        obligatory:u32,
        priorityInOrder:bool,
        coefficient:u64,
    )  {
        let attributes_in_queue = {
            AttributesInQueue:: new(
                key,
                QueueKey,
                &name,
                &typeAttribute,
                &order,
                sortable,
                obligatory,
                priorityInOrder,
                coefficient,
            )
        };
        // let mut history = self.queues_attr(&QueueKey);
        // history.push(attributes_in_queue);
        self.attributes_in_queues().put(key, attributes_in_queue);
    }
    ///method for adding attributes to queu
    pub fn create_profile (
        &mut self,
        key: &PublicKey,       
        user_key: PublicKey,
        queue_key: PublicKey,      
        rating: u64,
    )  {
        let profile = {
            Profile:: new(
                key,       
                user_key,
                queue_key,      
                rating,
            )
        };
        self.profiles().put(key, profile);
    }
    ///
    pub fn set_profile_attribute (
        &mut self,
        key: &PublicKey,       
        attribute_key: PublicKey,
        profile_key: PublicKey,      
        value: &String,
    ) {
        let profileValue = {
            ProfileAttributeValue:: new(
                key,       
                attribute_key,
                profile_key,      
                &value,
            )
        };
        self.profiles_attributes().put(key, profileValue);
    }
}
