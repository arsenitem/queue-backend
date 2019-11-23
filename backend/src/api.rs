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
    ///key to get queue
    pub pub_key: PublicKey,
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    //get queue by key
    fn get_queue(state: &ServiceApiState, query: GetFirstQuery) -> api::Result<String> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let first = schema.queue(&query.pub_key).unwrap();

        Ok(first.name)
    }
    // //get queue by key
    // fn get_all_queus(state: &ServiceApiState, _: GetAllQueues) -> api::Result<String> {
    //     let snapshot = state.snapshot();
    //     let schema = Schema::new(&snapshot);

    //     let allQueues = schema.queues();

    //     Ok(allQueues.values())
    // }
    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/queue_constructor/get_queue", Self::get_queue);
            // .endpoint("vq/queue_constructor/get_first", Self::get_first);
    }
   
}
