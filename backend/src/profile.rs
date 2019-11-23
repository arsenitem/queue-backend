use exonum::crypto::{Hash, PublicKey};

use super::proto;
/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Profile", serde_pb_convert)]
pub struct Profile {
    /// `PublicKey` of the profile.
    pub key: PublicKey,
    ///public key of user
    pub user_key: PublicKey,
    ///public key of queue
    pub queue_key: PublicKey,
    /// rating
    pub rating: u64,
}

impl Profile {
    /// Create new profile.
    pub fn new(
        &key: &PublicKey, 
        &user_key: &PublicKey,
        &queue_key: &PublicKey,      
        rating: u64,
    ) -> Self {
        Self {
            key,
            user_key,
            queue_key,
            rating,
        }
    }
}