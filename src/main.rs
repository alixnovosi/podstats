#[macro_use]
extern crate serde_derive;

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

    let conf = config.unwrap();

    let subs = config::load_cache(conf).unwrap();
    for (i, sub) in subs.iter().enumerate() {
        println!("sub {0}:\n{1}\n", i, sub);
    }
}
