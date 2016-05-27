extern crate rand;

use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Corpus {
    context: usize,
    words: Vec<String>,
    table: HashMap<Vec<String>, HashMap<String, f64>>,
}

pub struct Chain<'a> {
    history: VecDeque<String>,
    corpus: &'a Corpus,
}

impl Corpus {
    pub fn new(path: &Path, context: usize) -> io::Result<Corpus> {
        let mut f = try!(File::open(path));
        let mut buffer = String::new();
        try!(f.read_to_string(&mut buffer));

        let words: Vec<String> = buffer.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        let mut builder = HashMap::new();
        let mut history = VecDeque::new();
        for word in words.iter() {
            if history.len() == context {
                let word_list = builder.entry(history.clone()).or_insert(vec![]);
                (*word_list).push(word.clone());
                history.push_back(word.clone());
                history.pop_front();
            } else {
                history.push_back(word.clone());
            }
        }
        let mut table = HashMap::new();
        for (history, words) in builder.drain() {
            let mut word_count = HashMap::new();
            let total_words = words.len() as u32;
            for word in words {
                let count = word_count.entry(word).or_insert(0u32);
                *count += 1;
            }
            let mut word_probs = HashMap::new();
            for (word, count) in word_count.drain() {
                word_probs.insert(word, f64::from(count) / f64::from(total_words));
            }
            table.insert(history.into_iter().collect(), word_probs);
        }

        Ok(Corpus {
            context: context,
            words: words,
            table: table,
        })
    }

    pub fn words(&self, history: &[&str]) -> Option<Chain> {
        if history.len() != self.context { return None }
        Some(Chain {
            history: history.iter().map(|&s| s.to_lowercase()).collect(),
            corpus: self,
        })
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let key: Vec<String> = self.history.iter().cloned().collect();
        self.history.pop_front();
        match self.corpus.table.get(&key) {
            Some(word_probs) => {
                let r = rand::thread_rng().gen::<f64>();
                let mut acc = 0.0;
                for (word, prob) in word_probs {
                    acc += *prob;
                    if acc > r {
                        self.history.push_back(word.clone());
                        return Some(word)
                    }
                }
                unreachable!("failed to pick a word")
            }
            None => {
                let word = rand::thread_rng().choose(&self.corpus.words)
                    .expect("word shouldn't be empty");
                self.history.push_back(word.clone());
                Some(word)
            }
        }
    }
}
