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
    api::{ParticipantInfo , GetFirstQuery, ParticipantQuery},
    query::Quert,
    transactions::{CreateQueue},
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
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));

    // // Check that the user indeed is persisted by the service.
    let queue = api.get_queue(tx.author()).unwrap();
     assert_eq!(queue.pub_key, tx.author());
    assert_eq!(queue.name, SUPER_QUEUE);
    // assert_eq!(wallet.balance, 100);
}
fn test_add_attributes() {
    let (mut testkit, api) = create_testkit();
    // Create and send a transaction via API
    let (pk, _) = crypto::gen_keypair();
    let hello = String::from("Добавление атрибутов");
    let (tx, _) = api.add_attributes(pk: &PublicKey, queue.pub_key: &PublicKey, name: String, typeAttribute: String, order: u64, sortable: bool, obligatory: bool, priorityInOrder: bool, coefficient: u32)(&pk, hello);
    testkit.create_block();
    api.assert_tx_status(tx.hash(), &json!({ "type": "success" }));

    // // Check that the user indeed is persisted by the service.
    let attributesinqueue = api.get_attributesinqueue(attributesinqueue.pub_key).unwrap();
     assert_eq!(attributesinqueue.key, tx.author());
    assert_eq!(attributesinqueue.name, ATTRIBUTE.NAME);
    assert_eq!(attributesinqueue.typeAttribute, TYPEATTR);
    assert_eq!((attributesinqueue.order, 1);
    assert_eq!(attributesinqueue.sortable, true);
    assert_eq!(attributesinqueue.obligatory, true );
    assert_eq!(attributesinqueue.priorityInOrder, true );
    assert_eq!(attributesinqueue.coefficient, 1);
    // assert_eq!(wallet.balance, 100);
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
        let tx = CreateQueue::sign(&pubkey,pk,name,&key);

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
    fn add_attributes(&self,pk: &PublicKey, name: String, typeAttribute: String, order: u64, sortable: bool, obligatory: bool, priorityInOrder: bool, coefficient: u32 ) -> (Signed<RawTransaction>, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        // Create a pre-signed transaction
        let tx = AddAttributes::sign(&pubkey,pk,name, typeAttribute, order, sortable, obligatory, priorityInOrder, coefficient, &key);

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
}

/// Creates a testkit together with the API wrapper defined above.
fn create_testkit() -> (TestKit, ParticipantsApi) {
    let testkit = TestKitBuilder::validator().with_service(Service).create();
    let api = ParticipantsApi {
        inner: testkit.api(),
    };
    (testkit, api)
}
