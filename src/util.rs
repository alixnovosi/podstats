extern crate termion;

use std::fmt::Write as FmtWrite;
use std::io::{stdin, stdout, Stdout, Write};

// use self::termion::color;
use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::raw::IntoRawMode;
use self::termion::raw::RawTerminal;

pub struct Prompt {}

impl Prompt {
    pub fn select_from_menu(&self, menu_items: &Vec<&str>) -> Option<usize> {
        // Enter raw mode.
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        // Write to stdout (note that we don't use `println!`)
        // TODO find out how to get this to print properly.
        // should be blurb -> wait for key -> display result -> repeat
        write_helper(
            &mut stdout,
            "What do you want to do?(enter number, or q to quit)",
        );
        for (i, item) in menu_items.iter().enumerate() {
            // TODO this is bad
            let mut s = String::new();
            write!(&mut s, "{} {}", i + 1, item).unwrap();
            let v = s.clone();
            write_helper(&mut stdout, &v);
        }

        // Get a result.
        let mut res = String::new();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => {
                    stdout.flush().unwrap();
                    return None;
                }
                Key::Char('\n') => break,
                Key::Char(e) => {
                    let mut s = String::new();
                    write!(&mut s, "{}", e).unwrap();
                    let v = s.clone();

                    match write!(stdout, "{}", v) {
                        Ok(_) => (),
                        Err(why) => panic!("couldn't write to file: {}", why),
                    };

                    stdout.flush().unwrap();
                    res.push(e);
                }
                Key::Alt(_) => {}
                _ => {}
            }
        }

        write_helper(&mut stdout, "");
        stdout.flush().unwrap();
        match res.parse::<usize>() {
            Ok(n) => Some(n),
            Err(e) => {
                println!("{:#?}", e);
                println!("res: {:#?}", res);
                Some(0)
            }
        }
    }
}

fn write_helper(stdout: &mut RawTerminal<Stdout>, item: &str) {
    write!(stdout, "{}\n\r", item.to_string()).unwrap();
}
