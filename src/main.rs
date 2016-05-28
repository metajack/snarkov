extern crate clap;
extern crate snarkov;

use clap::{App, Arg};
use std::path::Path;
use std::str::FromStr;
use snarkov::Corpus;

fn format_seed(seed: [u32; 4]) -> String {
    format!("{:x}{:x}{:x}{:x}", seed[0], seed[1], seed[2], seed[3])
}

fn parse_seed(seed: &str) -> [u32; 4] {
    assert!(seed.len() == 32);
    let s1 = u32::from_str_radix(&seed[..8], 16).unwrap();
    let s2 = u32::from_str_radix(&seed[8..16], 16).unwrap();
    let s3 = u32::from_str_radix(&seed[16..24], 16).unwrap();
    let s4 = u32::from_str_radix(&seed[24..], 16).unwrap();
    [s1, s2, s3, s4]
}

fn main() {
    let matches = App::new("snarkov")
        .about("Generate markov chains from text")
        .arg(Arg::with_name("context")
             .help("Amount of context for prediction")
             .short("c")
             .long("context")
             .value_name("NUM")
             .default_value("2")
             .takes_value(true))
        .arg(Arg::with_name("length")
             .help("Minimum length of chain to produce")
             .short("l")
             .long("length")
             .value_name("LEN")
             .default_value("25")
             .takes_value(true))
        .arg(Arg::with_name("seed")
             .help("Random seed")
             .short("s")
             .long("seed")
             .value_name("SEED")
             .takes_value(true))
        .arg(Arg::with_name("INPUT")
             .help("Input text file")
             .required(true))
        .arg(Arg::with_name("start")
             .help("Starting words")
             .required(true)
             .min_values(1))
        .get_matches();

    let input = Path::new(matches.value_of("INPUT").unwrap());
    let context = match usize::from_str(matches.value_of("context").unwrap()) {
        Ok(v) => v,
        Err(_) => {
            println!("error: context was not an integer");
            ::std::process::exit(1);
        }
    };
    let length = match usize::from_str(matches.value_of("length").unwrap()) {
        Ok(v) => v,
        Err(_) => {
            println!("error: length was not an integer");
            ::std::process::exit(1);
        }
    };
    let start = matches.values_of("start").unwrap().collect::<Vec<_>>();

    let mut corpus = Corpus::new(input, context).unwrap();
    if let Some(seed) = matches.value_of("seed") {
        corpus.seed(parse_seed(seed));
    }
    println!("random seed = {}", format_seed(corpus.get_seed()));
    print!("{} ", start.join(" "));
    for (i, word) in corpus.words(&start).enumerate() {
        print!("{}", word);
        if i < length - 1 || !word.ends_with(".") {
            print!(" ");
        } else {
            break
        }
    }
}
