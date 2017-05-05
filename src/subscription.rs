extern crate serde;
extern crate rmp;
extern crate rmp_serde as rmps;

use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Subscription {
    pub url: String,
    pub original_url: String,
    pub name: String,
    pub directory: String,
    pub backlog_limit: u64,
    pub use_title_as_filename: bool,
    feed_state: FeedState,
}

impl Subscription {
    pub fn new(url: &str, name: &str, directory: Option<&str>) -> Subscription {

        Subscription {
            url: url.to_string(),
            original_url: url.to_string(),
            name: name.to_string(),
            directory: process_directory(directory),
            backlog_limit: 0,
            use_title_as_filename: false,

            feed_state: FeedState {
                latest_entry_number: 0,
                queue: Vec::new(),
                entries: Vec::new(),
                summary_queue: Vec::new(),
            },
        }
    }
}

impl fmt::Display for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn vec_serialize(subs: &Vec<Subscription>) -> Vec<u8> {
    let op_vec = rmps::to_vec(&subs);

    match op_vec {
        Ok(t) => return t,
        Err(_) => return Vec::new(),
    };
}

pub fn serialize(sub: &Subscription) -> Vec<u8> {
    let op_vec = rmps::to_vec(&sub);

    match op_vec {
        Ok(t) => return t,
        Err(_) => return Vec::new(),
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

pub fn vec_deserialize(sub_vec: &Vec<u8>) -> Option<Vec<Subscription>> {
    let slice: &[u8] = sub_vec.as_slice();

    let op_subs = rmps::from_slice(&slice);

    match op_subs {
        Ok(op_sub) => return Some(op_sub),
        Err(why) => {
            panic!("{:#?}", why);
            return None;
        }
    }
}

pub fn file_deserialize(path: &str) -> Option<Vec<Subscription>> {
    // Get path.
    let path = Path::new(&path);
    let display = path.display();

    // Open path in read-only mode.
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };

    // Read file contents into buffer.
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),

        Ok(_) => (),
    }

    return vec_deserialize(&buffer);
}

fn process_directory(directory: Option<&str>) -> String {
    match directory {
        // TODO expand given dir.
        Some(x) => return x.to_string(),
        // TODO properly default str.
        None => return "fakedir".to_string(),
    }
}

#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
struct FeedState {
    entries: Vec<Entry>,
    // entries_state_dict
    queue: Vec<Entry>,
    latest_entry_number: u64,
    summary_queue: Vec<SummaryEntry>,
    // last_modified
    // etag
}

#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
struct Entry {
    title: String,
    urls: Vec<String>,
}

#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
struct SummaryEntry {
    is_this_session: bool,
    number: u64,
    name: String,
}

impl fmt::Display for FeedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[test]
fn serialize_deserialize_test() {
    let sub = Subscription::new("testurl", "testname", None);
    let s = serialize(&sub);
    let re_sub = deserialize(&s);

    assert_eq!(sub, re_sub.unwrap());
}

#[test]
fn vec_serialize_deserialize_test() {
    let sub = Subscription::new("testurl", "testname", None);
    let mut subs = Vec::new();
    subs.push(sub);

    let s = vec_serialize(&subs);
    let re_subs = vec_deserialize(&s);

    assert_eq!(subs, re_subs.unwrap());
}

#[test]
fn file_serialize_test() {

    let test_path = "tmp_test.txt";

    // Get sub.
    let sub = Subscription::new("testurl", "testname", None);
    let mut subs = Vec::new();
    subs.push(sub);
    let s = vec_serialize(&subs);

    // Set up file.
    let path = Path::new(test_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(s.as_slice()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => (),
    }


    let sub_vec = file_deserialize(test_path).unwrap();

    assert_eq!(subs, sub_vec);

    fs::remove_file(test_path);
}
