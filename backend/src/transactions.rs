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
///struct createUser
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateUser")]
pub struct CreateUser{
    pub name: String,
}

///struct createQueue
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateQueue")]
pub struct CreateQueue {
    /// ads
    pub name: String,
}
///struct addattributes
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::AddAttributesToQueue")]
pub struct AddAttributesToQueue {
    pub name: String,
    pub  typeAttribute: String,
    pub order: Strin,
    pub sortable: uint64,
    pub obligatory: uint32,
    pub priorityInOrder: uint32,
    pub coefficient: uint32,
}
/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum ParticipantTransactions {
    ///
     CreateQueue(CreateQueue),
     CreateUser(CreateUser),
}
impl AddAttributesToQueue{
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        name: String,
        typeAttribute: String,
        order: u64,
        sortable: bool,
        obligatory: bool,
        priorityInOrder: bool,
        coefficient: u32,
        sk: &SecretKey,
    ) -> Signed<RawTransaction>{
        Message::sign_transaction(Self {name,  typeAttribute, order, sortable, obligatory, priorityInOrder, coefficient}, SERVICE_ID, *pk, sk)
    }    
}
impl Transaction for AddAttributesToQueue{
    fn execute(&self, context: TransactionContext) -> ExecutionResult{
        let mut schema = Schema:: new(context.fork());
        let key = &context.author();
        if schema.queue_attributes(key).is_none() {
            let name = &self.name;
            let  typeAttribute = &self. typeAttribute;
            let order = &self.order;
            let sortable = &self.sortable;
            let obligatory = &self.obligatory;
            let priorityInOrder = &self.priorityInOrder;
            let coefficient = &self.coefficient;
            schema.add_attributes(key, name, typeAttribute, order, sortable, obligatory, priorityInOrder, coefficient);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}
impl CreateUser{
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        name: String,
        sk: &SecretKey,
    ) -> Signed<RawTransaction>{
        Message::sign_transaction(Self {name}, SERVICE_ID, *pk, sk)
    }    
}
impl Transaction for CreateUser{
    fn execute(&self, context: TransactionContext) -> ExecutionResult{
        let mut schema = Schema:: new(context.fork());
        let key = &context.author();
        if schema.user(key).is_none() {
            let name = &self.name;

            schema.add_user(key, name);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
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
