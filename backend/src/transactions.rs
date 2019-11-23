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
    /// `PublicKey` of participant.
    pub key: PublicKey,
    /// ads
    pub name: String,
}
/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum ParticipantTransactions {
    ///
     CreateQueue(CreateQueue),
}


impl CreateQueue {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &key: &PublicKey,
        name: String,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key, name }, SERVICE_ID, *pk, sk)
    }
}
impl Transaction for CreateQueue {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();
    
        let mut schema = Schema::new(context.fork());

        let key = &self.key;

        if schema.queue(key).is_none() {
            let name = &self.name;

            schema.add_queue(key, name);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
     
    }
}
