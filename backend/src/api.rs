use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};
use exonum_merkledb::{ListProof, MapProof};

use super::{schema::Schema, SERVICE_ID};
use crate::queue::Queue;

/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GetFirstQuery {

}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope();
            // .endpoint("v1/queue_constructor/info", Self::participant_info)
            // .endpoint("vq/queue_constructor/get_first", Self::get_first);
    }
}
