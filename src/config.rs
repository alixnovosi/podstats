extern crate serde_yaml as yamls;
extern crate xdg;

use std::io::prelude::*;
use std::fmt;
use std::fs::File;
use std::path::Path;

use itertools::Itertools;

use subscription;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub cache_location: String,
    subscriptions: Vec<subscription::Subscription>,
}

impl Config {
    pub fn new(cache_location: Option<String>) -> Config {
        Config {
            cache_location: process_location(cache_location).to_string(),
            subscriptions: Vec::new(),
        }
    }

    pub fn load_cache(&mut self) {
        self.subscriptions = subscription::file_deserialize(&self.cache_location).unwrap()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.subscriptions
            .clone()
            .into_iter()
            .map(|s| s.name)
            .collect::<Vec<String>>()
    }

    pub fn get_entry_counts(&self) -> Vec<u64> {
        self.subscriptions
            .clone()
            .into_iter()
            .map(|s| s.get_latest_entry_number())
            .collect::<Vec<u64>>()
    }

    pub fn get_highest_entry_count_sub(&self) -> subscription::Subscription {
        self.subscriptions
            .clone()
            .into_iter()
            .sorted_by(|b, a| Ord::cmp(&a.get_latest_entry_number(), &b.get_latest_entry_number()))
            .into_iter()
            .collect::<Vec<subscription::Subscription>>()
            .first()
            .unwrap()
            .clone()
    }

    pub fn get_highest_entry_count_sub_name(&self) -> String {
        self.get_highest_entry_count_sub().name
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

fn process_location(cache_location: Option<String>) -> String {
    if cache_location != None {
        return cache_location.unwrap();
    }

    let xdg_dirs = xdg::BaseDirectories::with_prefix("puckfetcher").unwrap();

    let op_cache_file = xdg_dirs.find_cache_file("puckcache");
    if op_cache_file == None {
        panic!("No puckfetcher cache available");
    }

    let path = op_cache_file.unwrap();

    let file_str = path.to_str().unwrap().to_string();

    return file_str;
}

pub fn read_config() -> Option<Config> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("podstats").unwrap();

    let op_config_path = xdg_dirs.find_config_file("config.yaml");
    if op_config_path == None {
        return None;
    }
    let config_path = op_config_path.unwrap();

    let path = Path::new(&config_path);
    let display = path.display();

    // Open path in read-only mode.
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };

    // Read file contents into buffer.
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),

        Ok(_) => (),
    }

    let op_config = yamls::from_slice(buffer.as_slice());

    match op_config {
        Ok(config) => return Some(config),
        Err(_) => return None,
    }
}

pub fn write_config(config: Config) {
    let op_config = yamls::to_vec(&config);

    match op_config {
        Err(why) => panic!("couldn't encode config: {}", why),
        Ok(_) => (),
    }

    let xdg_dirs = xdg::BaseDirectories::with_prefix("podstats").unwrap();

    let mut op_config_path = xdg_dirs.find_config_file("config.yaml");
    if op_config_path == None {
        let path_res = xdg_dirs.place_config_file("config.yaml");
        match path_res {
            Err(why) => panic!("Couldn't find path for config: {}", why),
            Ok(_) => (),
        };

        // Create file.
        let config_path = path_res.unwrap();
        let path = Path::new(&config_path);
        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create file: {:?}", why),
            Ok(file) => file,
        };

        file.write_all(b"");
        op_config_path = xdg_dirs.find_config_file("config.yaml");
    }

    let config_path = op_config_path.unwrap();

    let path = Path::new(&config_path);
    let display = path.display();

    // TODO: proper error handling
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    let res = file.write(op_config.unwrap().as_slice());
    let bytes = match res {
        Ok(n) => n,
        Err(why) => panic!("couldn't write to file: {}", why),
    };

    file.flush();
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    use subscription;
    use config;

    fn setup_loaded_cache(subs: Option<Vec<subscription::Subscription>>) -> config::Config {
        let test_cache_loc = "testcache";
        let mut config = config::Config::new(Some(test_cache_loc.to_string()));

        // Allow providing subs, default if none are given.
        let unpacked_subs = match subs {
            Some(s) => s,
            None => {
                let sub1 = subscription::Subscription::new("testurl1", "testname1", None);
                let sub2 = subscription::Subscription::new("testurl2", "testname2", None);

                let mut subs = Vec::new();
                subs.push(sub1);
                subs.push(sub2);
                subs
            }
        };

        let s = subscription::vec_serialize(&unpacked_subs);

        // Set up file.
        let path = Path::new(test_cache_loc);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write subs to `file`, returns `io::Result<()>`
        match file.write_all(s.as_slice()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        config.load_cache();

        fs::remove_file(test_cache_loc);

        return config;
    }

    #[test]
    fn test_get_names() {
        let mut conf = setup_loaded_cache(None);

        let mut n = Vec::new();
        n.push("testname1");
        n.push("testname2");

        let names = conf.get_names();

        assert_eq!(n, names);
    }

    #[test]
    fn test_get_entry_counts() {
        let mut conf = setup_loaded_cache(None);

        let mut l_vec = Vec::new();
        l_vec.push(0);
        l_vec.push(0);

        let latest_vec = conf.get_entry_counts();

        assert_eq!(l_vec, latest_vec);
    }

    #[test]
    fn test_get_highest_entry_count_sub() {

        let sub1 = subscription::Subscription::new("testurl1", "testname1", None);
        let sub2 = subscription::Subscription::new("testurl2", "testname2", None);

        let mut subs = Vec::new();
        subs.push(sub1.clone());
        subs.push(sub2.clone());

        let mut conf = setup_loaded_cache(Some(subs));

        let sub = conf.get_highest_entry_count_sub();

        assert_eq!(sub1, sub);
    }

    #[test]
    fn test_get_highest_entry_count_sub_name() {

        let sub1 = subscription::Subscription::new("testurl1", "testname1", None);
        let sub2 = subscription::Subscription::new("testurl2", "testname2", None);

        let mut subs = Vec::new();
        subs.push(sub1.clone());
        subs.push(sub2.clone());

        let mut conf = setup_loaded_cache(Some(subs));

        let name = conf.get_highest_entry_count_sub_name();

        assert_eq!(sub1.name, name);
    }
}
