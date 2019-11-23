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
    pub QueueKey: PublicKey,
    /// Name of the field.
    pub name: String,   
    /// type    
    pub typeAttribute: String,  
    /// order
    pub order: u64,
    ///priority 
    pub sortable: bool,
    ///priority vector
    pub obligatory: bool,
    /// coefficient
    pub priorityInOrder: bool,
    ///required or not
    pub coefficient: u64,
}

impl AttributesInQueue {
    /// Create new queue.
    pub fn new(
        &key: &PublicKey,     
        QueueKey: PublicKey,        
        name: &str,
        typeAttribute: String,
        order: u64,
        sortable: bool,
        obligatory: bool,
        priorityInOrder: bool,
        coefficient: u64,
    ) -> Self {
        Self {
            key,
            QueueKey,
            name: name.to_owned(),
            typeAttribute,
            order,
            sortable,
            obligatory,
            priorityInOrder,
            coefficient,
        }
    }
}
   