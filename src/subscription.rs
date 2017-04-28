extern crate serde;
extern crate rmp;
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use self::serde::{Deserialize, Serialize};
use self::rmps::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Subscription {
    pub url: String,
    pub original_url: String,
    pub name: String,
    pub directory: String,
    pub backlog_limit: u64,
    temp_url: String,
    feed_state: FeedState,
}

impl Subscription {
    pub fn new(url: &str, name: &str, directory: Option<&str>) -> Subscription {

        Subscription{
            url: url.to_string(),
            original_url: url.to_string(),
            temp_url: "".to_string(),
            name: name.to_string(),
            directory: process_directory(directory),
            backlog_limit: 0,

            feed_state: FeedState{},
        }
    }
}

pub fn serialize(sub: &Subscription) -> Vec<u8> {
    let op_vec = rmps::to_vec(&sub);

    match op_vec {
        Ok(t) => return t,
        Err(_)  => return Vec::new(),
    };
}

pub fn deserialize(sub_vec: &Vec<u8>) -> Option<Subscription> {
    let slice: &[u8] = sub_vec.as_slice();

    let op_sub = rmps::from_slice(&slice);

    match op_sub {
        Ok(op_sub) => return Some(op_sub),
        Err(_) => return None,
    }
}

fn process_directory(directory: Option<&str>) -> String {
    match directory {
        // TODO expand given dir.
        Some(x) => return x.to_string(),
        // TODO properly default str.
        None    => return "fakedir".to_string(),
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
struct FeedState {

}

