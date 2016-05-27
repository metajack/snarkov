extern crate clap;
extern crate snarkov;

use clap::{App, Arg};
use std::path::Path;
use std::str::FromStr;
use snarkov::Corpus;

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
        .arg(Arg::with_name("INPUT")
             .help("Input text file")
             .required(true))
        .arg(Arg::with_name("seed")
             .help("Seed words (must be as many as context)")
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
    let seed = matches.values_of("seed").expect("foo").collect::<Vec<_>>();
    if seed.len() != context {
        println!("error: context is {} but {} seeds specified", context, seed.len());
        ::std::process::exit(1);
    }

    let corpus = Corpus::new(input, context).unwrap();
    print!("{} ", seed.join(" "));
    for (i, word) in corpus.words(&seed).unwrap().enumerate() {
        print!("{}", word);
        if i < length - 1 || !word.ends_with(".") {
            print!(" ");
        } else {
            break
        }
    }
}
