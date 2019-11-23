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

//!  queue.

use exonum::crypto::{Hash, PublicKey};

use super::proto;
use serde_json::json;

/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Queue", serde_pb_convert)]
pub struct Queue {
    /// `PublicKey` of the queue.
    pub queue_key: PublicKey,
    /// Name of the queue.
    pub name: String,   
}

impl Queue {
    /// Create new queue.
    pub fn new(
        &queue_key: &PublicKey,     
        name: &str,     
    ) -> Self {
        Self {
            queue_key,
            name: name.to_owned(),
        }
    }
   