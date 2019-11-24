#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::{proto, schema::Schema, SERVICE_ID};

/// Error codes emitted by pipes transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Participant already exists.
    ///
    /// Can be emitted by `Add`.
    #[fail(display = "Participant already exists")]
    ParticipantAlreadyExists = 0,
    // TODO add some errors
        /// Participant already removed.
    ///
    /// Can be emitted by `Remove`.
    #[fail(display = "Participant already removed")]
    ParticipantAlreadyRemoved = 1,

    /// Can't find participant by key.
    ///
    /// Can be emitted by `Remove`.
    #[fail(display = "Can't find participant by key")]
    ParticipantNotFound = 2,

    /// Participant already bought a phone.
    ///
    /// Can be emitted by `Buy`.
    #[fail(display = "Participant already bought a phone")]
    ParticipantAlreadyBought = 3,

    /// Participant is not first.
    ///
    /// Can be emitted by `Buy`.
    #[fail(display = "Participant is not first")]
    ParticipantIsNotFirst = 4
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

///struct
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateQueue")]
pub struct CreateQueue {
    /// ads
    pub name: String,
}
///struct
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::AddAttributesToQueue")]
pub struct AddAttributesToQueue {
    ///
    pub QueueKey: PublicKey,
    ///
    pub name: String,
    ///
    pub typeAttribute: String,
    ///
    pub order: String,
    ///
    pub sortable: u64,
    ///
    pub obligatory: u32 ,
    ///
    pub priorityInOrder: bool,
    ///
    pub coefficient: u64,
}
///struct
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateProfile")]
pub struct CreateProfile {
    ///
    pub user_key: PublicKey,
    ///
    pub queue_key: PublicKey,
    ///
    pub rating: u64,
}
///struct
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::SetProfileAttributeValue")]
pub struct SetProfileAttributeValue {
    ///
    pub attribute_key: PublicKey,
    ///
    pub profile_key: PublicKey,
    ///
    pub value: String,
}
/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum ParticipantTransactions {
    ///
     CreateQueue(CreateQueue),
    ///
     AddAttributesToQueue(AddAttributesToQueue),
     ///
     CreateProfile(CreateProfile),
    ///
     SetProfileAttributeValue(SetProfileAttributeValue),
}
///
impl AddAttributesToQueue {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        QueueKey: PublicKey,
        name: String,
        typeAttribute:String,
        order:String,
        sortable:u64,
        obligatory:u32,
        priorityInOrder:bool,
        coefficient:u64,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { QueueKey,name,typeAttribute, order, sortable, obligatory, priorityInOrder, coefficient}, SERVICE_ID, *pk, sk)
    }
}

impl CreateQueue {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,       
        name: String,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { name }, SERVICE_ID, *pk, sk)
    }
}
impl CreateProfile {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,       
        user_key: PublicKey,
        queue_key: PublicKey,      
        rating: u64,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { user_key, queue_key, rating }, SERVICE_ID, *pk, sk)
    }
}
impl SetProfileAttributeValue {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,       
        attribute_key: PublicKey,
        profile_key: PublicKey,      
        value: String,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { attribute_key, profile_key, value }, SERVICE_ID, *pk, sk)
    }
}
impl Transaction for CreateQueue {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        
    
        let mut schema = Schema::new(context.fork());

        let key = &context.author();

        if schema.queue(key).is_none() {
            let name = &self.name;

            schema.add_queue(key, name);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
     
    }
}
impl Transaction for AddAttributesToQueue {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
         
        let mut schema = Schema::new(context.fork());

        let key = &context.author();

            let QueueKey = self.QueueKey;
            let name = &self.name;
            let typeAttribute = &self.typeAttribute;
            let order = &self.order;
            let sortable = self.sortable;
            let obligatory = self.obligatory;
            let priorityInOrder = self.priorityInOrder;
            let coefficient = self.coefficient;
            schema.add_attributes_to_queue(
                key,
                QueueKey,
                name,
                typeAttribute,
                order,
                sortable,
                obligatory,
                priorityInOrder,
                coefficient
            );
            Ok(())
    }
}
impl Transaction for CreateProfile {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        
    
        let mut schema = Schema::new(context.fork());

        let key = &context.author();

        let user_key = self.user_key;
        let queue_key = self.queue_key; 
        let rating = self.rating;
        schema.create_profile(
            key,
            user_key,
            queue_key,      
            rating,
        );
        Ok(())
    }
}
impl Transaction for SetProfileAttributeValue {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        
    
        let mut schema = Schema::new(context.fork());

        let key = &context.author();

        let attribute_key = self.attribute_key;
        let profile_key = self.profile_key; 
        let value = &self.value;
        schema.set_profile_attribute(
            key,
            attribute_key,
            profile_key,      
            value,
        );
        Ok(())
    }
}
