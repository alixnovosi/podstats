#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate itertools;

extern crate clap;
extern crate termion;

use std::io::{Write, stdout, stdin};

use clap::{Arg, App, SubCommand};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod config;
mod subscription;

fn main() {
    let matches = App::new("podstats")
        .version("0.1")
        .author("Andrew Michaud <dev@mail.andrewmichaud.com")
        .about("Reads puckfetcher's msgpack cache and provides stats.")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
        .get_matches();

    println!("Loaded podstats !");

    let conf_file = match matches.value_of("config") {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    let mut conf = config::Config::new(conf_file);
    conf.load_cache();

    // Enter raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    let mut clear = false;

    // Actually do stuff:
    for c in stdin.keys() {

        // Write to stdout (note that we don't use `println!`)
        // TODO find out how to get this to print properly.
        // should be blurb -> wait for key -> display result -> repeat
        stdout.flush().unwrap();
        write!(stdout, "What do you want to do? (enter a number)\n\r").unwrap();
        write!(stdout, "\t1 provide names of podcasts in the cache\n\r").unwrap();
        write!(stdout,
               "\t2 provide latest entry numbers for podcasts in the cache\n\r")
                .unwrap();
        write!(stdout,
               "\t3 provide the full subscription in the cache with the most entries\n\r")
                .unwrap();
        write!(stdout,
               "\t4 provide the name of the subscription in the cache with the most entries\n\r")
                .unwrap();
        stdout.flush().unwrap();

        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "Quitting!\n\r").unwrap();
                break;
            }
            Key::Char('1') => {
                // TODO write a helper for this
                write!(stdout, "Subscription names:\n\r").unwrap();
                for (i, item) in conf.get_names().iter().enumerate() {
                    write!(stdout, "item {}: {}\n\r", i, item).unwrap();
                }
            }
            Key::Char('2') => {
                write!(stdout, "Subscription entry counts:\n\r").unwrap();
                for (i, item) in conf.get_entry_counts().iter().enumerate() {
                    write!(stdout, "item {} entry count: {}\n\r", i, item).unwrap();
                }
            }
            Key::Char('3') => {
                let item = conf.get_highest_entry_count_sub();
                // TODO handle sub indentation properly.
                // regex replace \n with \n\r ?
                write!(stdout, "Sub with highest entry count: {}\n\r", item).unwrap();
            }
            Key::Char('4') => {
                let item = conf.get_highest_entry_count_sub_name();
                write!(stdout, "Name of sub with highest entry count: {}\n\r", item).unwrap();
            }
            _ => write!(stdout, "who knows\n\r").unwrap(),
        }

        write!(stdout, "\n\r").unwrap();
    }
    // Here the destructor is automatically called, and the terminal state is restored.
}
