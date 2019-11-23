extern crate exonum;
extern crate exonum_configuration;
extern crate queue-constructor;

use exonum::helpers::{self, fabric::NodeBuilder};
use exonum_configuration as configuration;

fn main() {
    exonum::crypto::init();
    helpers::init_logger().unwrap();

    let node = NodeBuilder::new()
        .with_service(Box::new(configuration::ServiceFactory))
        .with_service(Box::new(queue-constructor::ServiceFactory));
    node.run();
}
