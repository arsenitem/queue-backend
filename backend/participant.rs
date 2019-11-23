use super::proto;
use exonum::crypto::{Hash, PublicKey};

/// Stores information about a participant
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Participant", serde_pb_convert)]
pub struct Participant {
    /// key
    pub key: PublicKey,
    ///json string
    pub serde_json: String,  
    ///key of the queue
    pub queue_key: PublicKey,
    /// took part
    pub took_part: bool,
    ///got a prize
    pub got_a_prize: bool,
    /// Removed
    pub removed: bool,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transactions history.
    pub history_hash: Hash,
}

impl Participant {
    /// Creates new participant
    pub fn new(
        &key: &PublicKey,  
        serde_json: &str,
        &queue_key: &PublicKey,      
        took_part: bool,
        got_a_prize: bool,        
        history_len: u64,
        &history_hash: &Hash,
    ) -> Self {
        Self {
            key,
            serde_json,
            queue_key,
            took_part,
            got_a_prize,            
            history_len,
            history_hash,
                }
    }
     /// take_part
     pub fn take_part(
        self,
        &history_hash: &Hash
    ) -> Self {
        &self.key,
        self.serde_json,
        &self.queue_key,
        true,
        self.get_a_prize,        
        self.history_len + 1,
        &history_hash
        )
    }
    /// get a prize
    pub fn get_a_prize(
        self,
        &history_hash: &Hash
    ) -> Self {
        Self::new(
            &self.key,
            self.serde_json,
            &self.queue_key,
            true,
            true,            
            self.history_len + 1,
            &history_hash
        )
    }
    
   
}
