#[macro_use]
extern crate serde_derive;
extern crate itertools;

extern crate clap;

use clap::{App, Arg};

mod config;
mod subscription;
mod util;

fn main() {
    let matches = App::new("podstats")
        .version("0.2.0")
        .author("Andrew Michaud <dev@mail.andrewmichaud.com")
        .about("Reads puckfetcher's msgpack cache and provides stats.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    println!("Loaded podstats!");

    let conf_file = match matches.value_of("config") {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    let mut conf = config::Config::new(conf_file);
    conf.load_cache();

    let prompt = util::Prompt {};

    let mut menu_options = Vec::new();
    menu_options.push("Get names of subscriptions in cache.");
    menu_options.push("Get entry counts of subscriptions in cache.");
    menu_options.push("Get sub with highest entry count.");
    menu_options.push("Get name of sub with highest entry count.");
    menu_options.push("Get the earliest entry for each sub.");
    menu_options.push("Get the latest entry for each sub.");

    loop {
        let res = prompt.select_from_menu(&menu_options);

        match res {
            Some(n) => {
                println!("{} was provided", n);
                // TODO provide fns to something to simplify this.
                match n {
                    1 => {
                        for (i, item) in conf.get_names().iter().enumerate() {
                            println!("{}: {}", i, item);
                        }
                    }
                    2 => {
                        for (i, item) in conf.get_entry_counts().iter().enumerate() {
                            println!("{} entry count: {}", i, item);
                        }
                    }
                    3 => {
                        let item = conf.get_highest_entry_count_sub();
                        println!("Sub with highest entry count: {}", item);
                    }
                    4 => {
                        let item = conf.get_highest_entry_count_sub_name();
                        println!("Name of sub with highest entry count: {}", item);
                    }
                    5 => {
                        for (i, item) in conf.get_earliest_entry_names().iter().enumerate() {
                            println!("{} earliest entry name: {}", i, item);
                        }
                    }
                    6 => {
                        for (i, item) in conf.get_latest_entry_names().iter().enumerate() {
                            println!("{} latest entry name: {}", i, item);
                        }
                    }
                    _ => println!("Given invalid option!"),
                }
            }

            None => {
                println!("Quitting!");
                return;
            }
        }

        println!("");
    }
}
