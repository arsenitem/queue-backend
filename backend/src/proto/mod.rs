#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

pub use self::queue_constructor::{User, CreateUser, AttributesInQueue, AddAttributesToQueue, Profile, ProfileAttributeValue, Query, CreateQuery};
include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::proto::schema::*;
