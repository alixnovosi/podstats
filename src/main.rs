#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate itertools;

mod config;
mod subscription;

fn main() {

    let config = config::read_config();
    println!("got config: {:?}", config);

    let c = config::Config::new(None);
    println!("made config: {:?}", c);

    config::write_config(c);

    let config = config::read_config();
    println!("got config: {:?}", config);

    let mut conf = config.unwrap();

    conf.load_cache();
    let longest = conf.get_longest();
    let longest_name = conf.get_longest_name();

    println!("longest: {0}\n{1}", longest_name, longest);
}
