extern crate rand;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::cmp;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Corpus {
    seed: [u32; 4],
    max_context: usize,
    words: Vec<String>,
    table: HashMap<Vec<String>, BTreeMap<String, f64>>,
}

pub struct Chain<'a> {
    rng: XorShiftRng,
    history: VecDeque<String>,
    corpus: &'a Corpus,
}

impl Corpus {
    pub fn new(path: &Path, max_context: usize) -> io::Result<Corpus> {
        let mut f = try!(File::open(path));
        let mut buffer = String::new();
        try!(f.read_to_string(&mut buffer));

        let words: Vec<String> = buffer.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        let mut builder = HashMap::new();
        let mut history = VecDeque::new();
        history.push_back(words[0].clone());
        for word in &words[1..] {
            let hist_len = history.len();
            for skip in 0..cmp::min(hist_len, max_context) {
                let key: Vec<String> = history.iter()
                    .skip(skip)
                    .cloned()
                    .collect();
                let word_list = builder.entry(key).or_insert(vec![]);
                word_list.push(word.clone());
            }

            history.push_back(word.clone());
            if history.len() > max_context {
                history.pop_front();
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
            let mut word_probs = BTreeMap::new();
            for (word, count) in word_count.drain() {
                word_probs.insert(word, f64::from(count) / f64::from(total_words));
            }
            table.insert(history.into_iter().collect(), word_probs);
        }

        let mut rng = rand::thread_rng();
        let seed = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];

        Ok(Corpus {
            seed: seed,
            max_context: max_context,
            words: words,
            table: table,
        })
    }

    pub fn seed(&mut self, seed: [u32; 4]) {
        self.seed = seed;
    }

    pub fn get_seed(&self) -> [u32; 4] {
        self.seed
    }

    pub fn words(&self, start: &[&str]) -> Chain {
        Chain {
            rng: SeedableRng::from_seed(self.seed),
            history: start.iter()
                .map(|&s| s.to_lowercase())
                .collect(),
            corpus: self,
        }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        while self.history.len() > self.corpus.max_context {
            self.history.pop_front();
        }

        let hist_len = self.history.len();
        if hist_len > 0 {
            for skip in 0..hist_len {
                let key: Vec<String> = self.history.iter()
                    .skip(skip)
                    .cloned()
                    .collect();
                match self.corpus.table.get(&key) {
                    Some(word_probs) => {
                        let r = self.rng.gen::<f64>();
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
                    None => {}
                }
            }
        }

        let word = self.rng.choose(&self.corpus.words)
            .expect("corpus words shouldn't be empty");
        self.history.push_back(word.clone());
        Some(word)
    }
}
