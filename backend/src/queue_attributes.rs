// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cryptocurrency queue fields.

use exonum::crypto::{Hash, PublicKey};

use super::proto;
use serde_json::json;

/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Queue_fields", serde_pb_convert)]
pub struct Queue_fields {
    /// `PublicKey` of the queue.
    pub queue_key: PublicKey,
    /// Name of the field.
    pub name: String,   
    /// type    
    pub type: String,  
    /// order
    pub order: String,
    //priority 
    pub priority: u64,
    ///priority vector
    pub priorityInOrder: bool,
    /// coefficient
    pub coefficient: u64;
    ///required or not
    pub required: u32,
    /// `Hash` of the transactions history.    
    //pub history_hash: Hash,
    ///Json string
    //pub serde_json: String,
    ///Creation date  
    //pub created: u64,
     ///Opening date  
    // pub opened: u64,
      ///Closing date  
   // pub closed: u64,   
    /// Length of the transactions history.
    //pub history_len: u64,
}

impl Queue_attributes {
    /// Create new queue.
    pub fn new(
        &queue_key: &PublicKey,        
        name: &str,
        type: u32,
        order: u32,
        priority: u32,
        priority_vector: u32,
        type: u32,
        coefficient: u32,
       // history_len: u64,
        //&history_hash: &Hash,
    ) -> Self {
        Self {
            queue_key,
            name: name.to_owned(),
            type,
            order,
            priority,
            priority_vector,
            type,
            coefficient,    
        }
    }
   