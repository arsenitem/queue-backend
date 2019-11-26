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
use crate::profile::Profile;
use crate::profile_attribute_value::ProfileAttributeValue;
use crate::queue_attributes::AttributesInQueue;

/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GetFirstQuery {
    ///key to get queue
    pub pub_key: PublicKey,
}
/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GetQueuesQuery {
}
/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GetProfilesQuery {
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
    fn get_queue_properties(state: &ServiceApiState, query: GetQueuesQuery) -> api::Result<Vec<AttributesInQueue>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let history = schema.attributes_in_queues();
        let mut vec = Vec::new();
        for value in history.values() {
            vec.push(value);
        }
        Ok(vec)
    }
    // //get queue by key
    fn get_all_queues(state: &ServiceApiState, _: GetQueuesQuery) -> api::Result<Vec<Queue>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let allQueues = schema.queues();
        let mut vec = Vec::new();
        for value in allQueues.values() {
            vec.push(value);
        }
        Ok(vec)
    }
    fn get_profiles(state: &ServiceApiState, _: GetQueuesQuery) -> api::Result<Vec<Profile>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let allprofiles = schema.profiles();
        let mut vec = Vec::new();
        for value in allprofiles.values() {
            vec.push(value);
        }
        vec.sort_by(|a, b| b.rating.cmp(&a.rating));
        Ok(vec)
    }
    fn get_profile_attributes(state: &ServiceApiState, _: GetQueuesQuery) -> api::Result<Vec<ProfileAttributeValue>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(&snapshot);

        let allprofiles = schema.profiles_attributes();
        let mut vec = Vec::new();
        for value in allprofiles.values() {
            vec.push(value);
        }
        Ok(vec)
    }
    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/queue_constructor/get_queue", Self::get_queue)
            .endpoint("v1/queue_constructor/get_queue_properties", Self::get_queue_properties)
            .endpoint("v1/queue_constructor/get_all_queues", Self::get_all_queues)
            .endpoint("v1/queue_constructor/get_profiles", Self::get_profiles)
            .endpoint("v1/queue_constructor/get_profile_attributes", Self::get_profile_attributes);
    }
}
