extern crate serde_yaml as yamls;
extern crate xdg;

use std::io::prelude::*;
use std::fmt;
use std::fs::File;
use std::path::Path;

use subscription;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub cache_location: String,
}

impl Config {
    pub fn new(cache_location: Option<String>) -> Config {
        Config { cache_location: process_location(cache_location).to_string() }
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

pub fn load_cache(config: Config) -> Option<Vec<subscription::Subscription>> {
    return subscription::file_deserialize(&config.cache_location);
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    use subscription;
    use config;

    #[test]
    fn test_load() {
        let test_cache_loc = "testcache";
        let config = config::Config { cache_location: test_cache_loc.to_string() };

        let sub = subscription::Subscription::new("testurl", "testname", None);
        let mut subs = Vec::new();
        subs.push(sub);
        let s = subscription::vec_serialize(&subs);

        // Set up file.
        let path = Path::new(test_cache_loc);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(s.as_slice()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        let re_subs = config::load_cache(config).unwrap();

        assert_eq!(subs, re_subs);

        fs::remove_file(test_cache_loc);
    }
}
