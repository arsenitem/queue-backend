use exonum::crypto::{Hash, PublicKey};

use super::proto;
/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::ProfileAttributeValue", serde_pb_convert)]
pub struct ProfileAttributeValue {
    /// `PublicKey` of the profile.
    pub key: PublicKey,
    ///public key of user
    pub attribute_key: PublicKey,
    ///public key of queue
    pub profile_key: PublicKey,
    /// rating
    pub value: String,
}

impl ProfileAttributeValue {
    /// Create new profile.
    pub fn new(
        &key: &PublicKey, 
        attribute_key: PublicKey,
        profile_key: PublicKey,      
        value: &String,
    ) -> Self {
        Self {
            key,
            attribute_key,
            profile_key,
            value: value.to_owned(),
        }
    }
}