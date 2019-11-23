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

/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::AttributesInQueue", serde_pb_convert)]
pub struct AttributesInQueue {
    ///pub key
    pub key: PublicKey,
    /// `PublicKey` of the queue.
    pub queueKey: PublicKey,
    /// Name of the field.
    pub name: String,   
    /// type    
    pub attr_type: String,  
    /// order
    pub order: String,
    //priority 
    pub priority: u64,
    ///priority vector
    pub priorityInOrder: bool,
    /// coefficient
    pub coefficient: u64,
    ///required or not
    pub required: u32,
}

impl AttributesInQueue {
    /// Create new queue.
    pub fn new(
        &key: &PublicKey,     
        queuekey: PublicKey,        
        name: &str,
        attr_type: String,
        order: String,
        priority: u64,
        required: u32,
        priorityInOrder: bool,
        coefficient: u64,
    ) -> Self {
        Self {
            key,
            queueKey: queueKey.to_owned(),
            name: name.to_owned(),
            attr_type,
            order,
            priority,
            required,
            priorityInOrder,
            coefficient,
        }
    }
}
   