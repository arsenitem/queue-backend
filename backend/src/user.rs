use exonum::crypto::{Hash, PublicKey};

use super::proto;
/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::User", serde_pb_convert)]
pub struct User {
    /// `PublicKey` of the user.
    pub key: PublicKey,
    /// Name of the user.
    pub name: String,
}

impl User {
    /// Create new user.
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