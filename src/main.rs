use clap::{App, Arg};
use regex::Regex;
use std::{
    fs::File,
    intrinsics::atomic_singlethreadfence_acqrel,
    io::{prelude::*, BufReader},
};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => print!("{}", line),
            None => {}
        }
    }
}

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
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
