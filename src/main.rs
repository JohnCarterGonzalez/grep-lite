use clap::{App, Arg};
use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Grep patterns, string search tool")
        .arg(
            Arg::with_name("needle")
                .help("The Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let needle = args.value_of("needle").unwrap();
    // unwrap a Result, panicing out those errors tho
    let re = Regex::new(needle).unwrap();

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => print!("{}", line),
            None => {}
        }
    }
}
