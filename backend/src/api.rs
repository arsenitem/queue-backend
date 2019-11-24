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
use crate::queue_attributes::AttributesInQueue;

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
    fn get_queue_properties(state: &ServiceApiState, query: GetFirstQuery) -> api::Result<Vec<AttributesInQueue>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let history = schema.queues_attr(&query.pub_key);
        // let first = schema.attributes_in_queues();
        // let mut vec = Vec::new();
        // for val in first.values() {
        //     vec.push(val.name);
        // }
        let transactions = history
        .iter()
        .collect::<Vec<_>>();

        // Ok(first.values().count())
         Ok( transactions)
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
            .endpoint("v1/queue_constructor/get_queue", Self::get_queue)
            .endpoint("v1/queue_constructor/get_queue_properties", Self::get_queue_properties);
    }
   
}
