//! Iphone queue

#![deny(
    missing_debug_implementations,
    missing_docs,
    unsafe_code,
    bare_trait_objects
)]

#[macro_use]
extern crate exonum_derive;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate protobuf;

/// API
pub mod api;
/// Pipe type struct
///doc
pub mod queue;
/// PB structures
pub mod proto;
/// Schema
pub mod schema;
/// Transactions
pub mod transactions;
/// atrs in queue
pub mod queue_attributes;
/// user profiles
pub mod profile;
///
pub mod profile_attribute_value;

///users
pub mod user;

use exonum_merkledb::Snapshot;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{self, Transaction, TransactionSet},
    crypto::Hash,
    helpers::fabric::{self, Context},
    messages::RawTransaction,
};

use schema::Schema;
use transactions::ParticipantTransactions;

/// Unique service id
pub const SERVICE_ID: u16 = 10;
/// Name of the service.
const SERVICE_NAME: &str = "queue_constructor";

/// Exonum `Service` implementation.
#[derive(Default, Debug)]
pub struct Service;

impl blockchain::Service for Service {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn state_hash(&self, view: &dyn Snapshot) -> Vec<Hash> {
        let schema = Schema::new(view);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        ParticipantTransactions::tx_from_raw(raw).map(Into::into)
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        api::PublicApi::wire(builder);
    }
}

/// A configuration service creator for the `NodeBuilder`.
#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<dyn blockchain::Service> {
        Box::new(Service)
    }
}
