#[macro_use]
extern crate serde_json;

use exonum::{
    api::node::public::explorer::{TransactionQuery, TransactionResponse},
    crypto::{self, Hash, PublicKey, SecretKey},
    messages::{self, RawTransaction, Signed},
};
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

// Import data types used in tests from the crate where the service is defined.
use queue_constructor::{
    api::{GetFirstQuery},
    queue::Queue,
    queue_attributes::AttributesInQueue,
    transactions::{CreateQueue, AddAttributesToQueue, CreateProfile},
    Service,
};

/// Check that the wallet creation transaction works when invoked via API.
#[test]
fn test_create_queue() {
    let (mut testkit, api) = create_testkit();
    // Create and send a transaction via API
    let (pk, _) = crypto::gen_keypair();
    let hello = String::from("Покупка билетов на концерт");
    let (tx, _) = api.create_queue(&pk, hello);
    let (pk2, _) = crypto::gen_keypair();
    let hello2 = String::from("Покупка билетов на концерт");
    let (tx2, _) = api.create_queue(&pk2, hello2);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));
    api.assert_tx_status(tx2.hash(), &json!({ "type": "success" }));

    // // Check that the user indeed is persisted by the service.
    // let wallet = api.get_wallet(tx.author()).unwrap();
    // assert_eq!(wallet.pub_key, tx.author());
    // assert_eq!(wallet.name, ALICE_NAME);
    // assert_eq!(wallet.balance, 100);
}
#[test]
fn create_attributes() {
    let (mut testkit, api) = create_testkit();
    // Create and send a transaction via API
    let (pk, _) = crypto::gen_keypair();
    let hello = String::from("Покупка билетов на концерт");
    let (tx, _) = api.create_queue(&pk, hello);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));
    let (pk2, _) = crypto::gen_keypair();
    let hello1 = String::from("Покупка билетов на концерт");
    let hello2 = String::from("Покупка билетов на концерт");
    let hello3 = String::from("Покупка билетов на концерт");
    let hello11 = String::from("Покупка билетов на концерт");
    let hello22 = String::from("Покупка билетов на концерт");
    let hello33 = String::from("Покупка билетов на концерт");
    let (tx2, _) = api.create_queue_attrs(pk, hello1, hello2, hello3, 5, 4, false, 5);
    let (tx3, _) = api.create_queue_attrs(pk, hello11, hello22, hello33, 5, 5, true, 6);
    testkit.create_block();
    let vec = api.get_queue_attrs(pk);
    api.assert_tx_status(tx2.hash(), &json!({ "type": "success" }));
    api.assert_tx_status(tx3.hash(), &json!({ "type": "success" }));
    assert_eq!(2, vec.len());

}
#[test]
fn create_profile() {
    let (mut testkit, api) = create_testkit();
    // Create and send a transaction via API
    let (pk, _) = crypto::gen_keypair();
    let (pk1, _) = crypto::gen_keypair();
    let (tx, _) = api.create_profile_attr(pk, pk1, 50);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));
    // let vec = api.get_queue_attrs(&pk);
}
/// Wrapper for the cryptocurrency service API allowing to easily use it
/// (compared to `TestKitApi` calls).
struct ParticipantsApi {
    pub inner: TestKitApi,
}

impl ParticipantsApi {
    /// Generates a wallet creation transaction with a random key pair, sends it over HTTP,
    /// and checks the synchronous result (i.e., the hash of the transaction returned
    /// within the response).
    /// Note that the transaction is not immediately added to the blockchain, but rather is put
    /// to the pool of unconfirmed transactions.
    // fn get_queues() -> Vec<Queue>{
    //     let info: serde_json::Value = self
    //         .inner
    //         .public(ApiKind::Service("queue_constructor"))
    //         .query()
    //         .get::<Vec<Queue>>("v1/queue_constructor/get_all_queues");
    //     info
    // }

    /// Asserts that the transaction with the given hash has a specified status.
    fn assert_tx_status(&self, tx_hash: Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self
            .inner
            .public(ApiKind::Explorer)
            .query(&TransactionQuery::new(tx_hash))
            .get("v1/transactions")
            .unwrap();

        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }
    fn create_queue(&self,pk: &PublicKey, name: String) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        // Create a pre-signed transaction
        let tx = CreateQueue::sign(&pubkey,name,&key);

        let data = messages::to_hex_string(&tx);
        let tx_info: TransactionResponse = self
            .inner
            .public(ApiKind::Explorer)
            .query(&json!({ "tx_body": data }))
            .post("v1/transactions")
            .unwrap();
        assert_eq!(tx_info.tx_hash, tx.hash());
        (tx, key)
    }
    fn create_queue_attrs(&self,
        QueueKey: PublicKey,
        name: String,
        typeAttribute:String,
        order:String,
        sortable:u64,
        obligatory:u32,
        priorityInOrder:bool,
        coefficient:u64,) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        // Create a pre-signed transaction
        let tx = AddAttributesToQueue::sign(&pubkey,QueueKey,name,typeAttribute, order, sortable, obligatory, priorityInOrder, coefficient,&key);

        let data = messages::to_hex_string(&tx);
        let tx_info: TransactionResponse = self
            .inner
            .public(ApiKind::Explorer)
            .query(&json!({ "tx_body": data }))
            .post("v1/transactions")
            .unwrap();
        assert_eq!(tx_info.tx_hash, tx.hash());
        (tx, key)
    }
    fn create_profile_attr(&self,
        user_key: PublicKey,
        queue_key: PublicKey,      
        rating: u64,
    )-> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        // Create a pre-signed transaction
        let tx = CreateProfile::sign(&pubkey,user_key,queue_key,rating,&key);

        let data = messages::to_hex_string(&tx);
        let tx_info: TransactionResponse = self
            .inner
            .public(ApiKind::Explorer)
            .query(&json!({ "tx_body": data }))
            .post("v1/transactions")
            .unwrap();
        assert_eq!(tx_info.tx_hash, tx.hash());
        (tx, key)
    }
    fn get_queue_attrs(&self, pub_key: PublicKey)-> Vec<AttributesInQueue> {
        let first_key = self
            .inner
            .public(ApiKind::Service("queue_constructor"))
            .query(&GetFirstQuery {pub_key})
            .get::<Vec<AttributesInQueue>>("v1/queue_constructor/get_queue_properties")
            .unwrap();
            
        println!("{:?}", first_key);
        first_key
    }
}

/// Creates a testkit together with the API wrapper defined above.
fn create_testkit() -> (TestKit, ParticipantsApi) {
    let testkit = TestKitBuilder::validator().with_service(Service).create();
    let api = ParticipantsApi {
        inner: testkit.api(),
    };
    (testkit, api)
}
