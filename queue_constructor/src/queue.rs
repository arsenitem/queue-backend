use exonum::crypto::{Hash, PublicKey};

use super::proto;
/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Queue", serde_pb_convert)]
pub struct Queue {
    /// `PublicKey` of the queue.
    pub key: PublicKey,
    /// Name of the queue.
    pub name: String,
}

impl Queue {
    /// Create new queue.
    pub fn new(
        &key: &PublicKey,     
        name: &String,     
    ) -> Self {
        Self {
            key,
            name: name.to_owned(),
        }
    }
}
   