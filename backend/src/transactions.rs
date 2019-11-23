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
    
    /// Participant already took part.
    ///
    /// Can be emitted by `Take_part`.
    #[fail(display = "Participant already took part")]
    ParticipantAlreadyTook_part = 1,

    /// Can't find participant by key.
    ///
    /// Can be emitted by `Take_part`.
    #[fail(display = "Can't find participant by key")]
    ParticipantNotFound = 2,

    /// Participant already won a prize.
    ///
    /// Can be emitted by `Win_a_prize`.
    #[fail(display = "Participant already won a prize")]
    ParticipantAlreadyWon = 3,

    /// Participant is not first.
    ///
    /// Can be emitted by `Win_a_prize`.
    #[fail(display = "Participant is not first")]
    ParticipantIsNotFirst = 4
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

/// Create participant.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Add")]
pub struct Add {
    /// `PublicKey` of participant.
    pub key: PublicKey,
    /// json data
    pub serde_json: String,
    ///public key of the queue
    pub queue_key: PublicKey,

}
/// Take part
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Take_part")]
pub struct Take_part {
    /// `PublicKey` of participant.
    pub key: PublicKey,
}

/// Get a prize
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Get_a_prize")]
pub struct Get_a_prize {
    /// `PublicKey` of participant.
    pub key: PublicKey,
}

/// Get a prize
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateQueue")]
pub struct CreateQueue {
    /// `PublicKey` of participant.
    pub key: PublicKey,
     
    pub name: String,
}


/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum ParticipantTransactions {
    /// Add tx.
    Add(Add),
    ///Take part
    Take_part(Take_part),
    /// Get a prize
    Get_a_prize(Get_a_prize),
    CreateQueue(CreateQueue)
   
}

impl Add {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &key: &PublicKey,
        serde_json: &str,
        &queue_key: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key, serde_json, queue_key, }, SERVICE_ID, *pk, sk)
    }
}

impl Take_part {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &key: &PublicKey,
        serde_json: &str,
        &queue_key: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key, serde_json, queue_key }, SERVICE_ID, *pk, sk)
    }
}

impl Get_a_prize {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &key: &PublicKey,
        serde_json: &str,
        &queue_key: &PublicKey,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { key, serde_json, queue_key }, SERVICE_ID, *pk, sk)
    }
}

impl Transaction for Add {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let key = &self.key;

        if schema.participant(key).is_none() {
            //let timestamp = self.timestamp;
            schema.add_participant(key, serde_json, queue_key, false, false, &hash);
            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
    }
}
impl Transaction for Take_part {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();
        let mut schema = Schema::new(context.fork());
        let key = &self.key;

        if let Some(participant) = schema.participant(key) {

            if participant.took_part {
                Err(Error::ParticipantAlreadyTook_part)?
            }

            schema.take_part(participant, &hash);
            Ok(())
        } else {
            Err(Error::ParticipantNotFound)?
        }
    }
}

impl Transaction for Win_a_prize {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();
        let mut schema = Schema::new(context.fork());
        let key = &self.key;

        if let Some(participant) = schema.participant(key) {
            if participant.won_a_prize {
                Err(Error::ParticipantAlreadyWon)?
            }
            
            let first = schema.first_participant().unwrap();
            if !first.key.eq(&participant.key) {
                Err(Error::ParticipantIsNotFirst)?
            }

            schema.participant_won_a_prize(participant, &hash);
            Ok(())
        } else {
            Err(Error::ParticipantNotFound)?
        }
    }
}
impl Transaction for CreateQueue {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let hash = context.tx_hash();
    
        let mut schema = Schema::new(context.fork());

        let key = &self.key;

        if schema.queue(key).is_none() {
            let name = self.name;

            schema.add_queue(key, name);

            Ok(())
        } else {
            Err(Error::ParticipantAlreadyExists)?
        }
     
    }
}
