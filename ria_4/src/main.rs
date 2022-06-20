use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }

    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true),
        )
        .get_matches();
    // get pattern value from args
    let pattern = args.value_of("pattern").unwrap();
    // set pattern with regex
    let re = Regex::new(pattern).unwrap();
    // get input value (file to read) from args
    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
    // open given file from input
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}, ", line),
            None => (),
        }
    }
}
