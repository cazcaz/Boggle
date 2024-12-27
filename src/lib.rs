use rand::Rng;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::time::{Duration, Instant};

pub mod utils;

#[derive(Clone)]
pub struct Boggle {
    array: Vec<Vec<char>>,
    possible_words: HashSet<String>,
}

impl Boggle {
    pub fn new() -> Self {
        let mut dice = vec![];
        let mut rand = rand::thread_rng();
        for _ in 0..4 {
            let mut cur: Vec<char> = vec![];
            for _ in 0..4 {
                let rand_letter: char = rand.gen_range(b'A'..b'Z') as char;
                cur.push(rand_letter);
            }
            dice.push(cur);
        }
        let mut boggle = Self {
            array: dice,
            possible_words: HashSet::new(),
        };
        boggle.find_all_words();
        boggle
    }

    pub fn get_possible_words(&self) -> HashSet<String> {
        self.possible_words.clone()
    }

    fn find_all_words(&mut self) -> Vec<String> {
        let result: Vec<String> = vec![];
        let mut dictionary = match utils::TrieManager::load_trie() {
            Ok(dictionary) => dictionary,
            Err(e) => {
                panic!("Failed to load or create the Trie: {}", e);
            }
        };

        for y in 0..4 {
            for x in 0..4 {
                let starting_letter: String = String::from(self.array[y][x]);
                if dictionary.extend_word(&starting_letter).len() > 0 {
                    let seen_indices = HashSet::<i32>::new();
                    let empty = String::new();
                    self.step_and_search((x, y), &seen_indices, &empty, &dictionary);
                }
            }
        }
        result
    }

    fn step_and_search(
        &mut self,
        loc: (usize, usize),
        seen: &HashSet<i32>,
        cur: &String,
        dictionary: &utils::DictTrie::DictTrie,
    ) {
        let mut new_cur = cur.clone();
        new_cur.push(self.array[loc.1][loc.0]);
        if dictionary.check_word(&new_cur) {
            if new_cur.len() > 2 {
                self.possible_words.insert(new_cur.clone());
            }
        }
        if dictionary.extend_word(&new_cur).len() == 0 {
            return;
        }
        let mut new_seen = seen.clone();
        new_seen.insert(Self::hash(loc.0, loc.1));
        let steps: Vec<(i32, i32)> = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        for step in steps {
            let new_pos = (loc.0 as i32 + step.0, loc.1 as i32 + step.1);
            if new_pos.0 < 0 || new_pos.0 > 3 {
                continue;
            }
            if new_pos.1 < 0 || new_pos.1 > 3 {
                continue;
            }
            let hash_val = Self::hash(new_pos.0.try_into().unwrap(), new_pos.1.try_into().unwrap());
            if new_seen.contains(&hash_val) {
                continue;
            }
            self.step_and_search(
                (new_pos.0.try_into().unwrap(), new_pos.1.try_into().unwrap()),
                &new_seen,
                &new_cur,
                dictionary,
            );
        }
    }

    fn hash(x: usize, y: usize) -> i32 {
        4 * y as i32 + x as i32
    }
}

impl fmt::Display for Boggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.array {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

pub struct Game {
    boggle: Boggle,
    found_words: HashSet<String>,
}
impl Game {
    pub fn new() -> Self {
        let mut boggle = Boggle::new();
        while boggle.get_possible_words().len() < 60 {
            boggle = Boggle::new();
        }
        Self {
            boggle,
            found_words: HashSet::new(),
        }
    }
    pub fn start(&mut self) {
        println!("{}", self.boggle);
        println!("Game started! Enter as many words as you can in 90 seconds.");
        let start_time = Instant::now();
        let stdin = io::stdin();
        let mut input = String::new();
        while start_time.elapsed() < Duration::new(90, 0) {
            input.clear();
            stdin.read_line(&mut input).expect("Failed to read line");
            let word = input.trim().to_uppercase();
            if self.boggle.get_possible_words().contains(&word) {
                if self.found_words.contains(&word) {
                    println!("You have already found this word. Try Again!");
                } else {
                    self.found_words.insert(word);
                }
            } else {
                println!("Not a valid word. Try again!");
            }
        }
        println!();
        println!("Final scores:");
        println!();
        println!(
            "You found {} of a total {} possible words",
            self.found_words.len(),
            self.boggle.get_possible_words().len()
        );

        let mut found_word_vec: Vec<&String> = self.found_words.iter().collect();
        let mut score = 0;
        let mut max_score = 0;
        found_word_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        for word in found_word_vec {
            score += word.len() - 2;
            println!("{} {}", word, word.len() - 2);
        }
        let mut possible_word_vec: Vec<&String> = self
            .boggle
            .possible_words
            .difference(&self.found_words)
            .collect();
        possible_word_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        for word in &possible_word_vec {
            max_score += word.len() - 2;
        }
        println!("");
        println!("You could have found some of these words: ");
        for i in 0..15 {
            if i >= possible_word_vec.len() {
                continue;
            }
            println!(
                "{} {}",
                possible_word_vec[i],
                possible_word_vec[i].len() - 2
            );
        }

        println!("");
        println!("Your final score: {} of {}", score, max_score);
    }
}
