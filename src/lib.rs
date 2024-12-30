use boggle_utils::boggle_board::BoggleBoard;
use rayon::prelude::*;
use serde_json::map::Iter;
use std::collections::HashSet;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod utils;

pub mod boggle_utils;

#[derive(Clone)]
pub struct BoggleSolver {
    board: BoggleBoard,
    possible_words: HashSet<String>,
    board_size: i32,
    diagonals: bool,
    dictionary: utils::dict_trie::DictTrie,
    multi_thread: bool,
}

impl BoggleSolver {
    pub fn new(
        board_size: i32,
        diagonals: bool,
        dictionary_path: String,
        multi_thread: bool,
    ) -> Self {
        let mut boggle_board = Self {
            board: BoggleBoard::new(board_size),
            possible_words: HashSet::new(),
            board_size,
            diagonals,
            dictionary: utils::trie_manager::load_trie(dictionary_path)
                .unwrap_or_else(|e| panic!("Failed to load or create the Trie: {}", e)),
            multi_thread,
        };
        boggle_board.store_all_words();
        boggle_board
    }

    pub fn get_possible_words(&self) -> HashSet<String> {
        self.possible_words.clone()
    }

    fn store_all_words(&mut self) {
        if self.possible_words.len() > 0 {
            self.possible_words = HashSet::new();
        }

        for word in self.find_all_words() {
            self.possible_words.insert(word);
        }
    }

    fn find_all_words(&self) -> Vec<String> {
        let board_size = self.board_size;

        let map_fn = |i: i32| -> Vec<String> {
            let y = i / board_size;
            let x = i % board_size;
            let mut starting_letter = String::new();
            self.board
                .access((y as usize, x as usize))
                .append_to(&mut starting_letter);
            if self.has_extension(&starting_letter) {
                let seen_indices = HashSet::<(i32, i32)>::new();
                let mut result = Vec::new();
                self.step_and_search(
                    (x as usize, y as usize),
                    &seen_indices,
                    &mut result,
                    &mut vec![],
                );
                result
            } else {
                vec![]
            }
        };

        let results: Vec<Vec<String>> = if self.multi_thread {
            (0..board_size * board_size)
                .into_par_iter()
                .map(map_fn)
                .collect()
        } else {
            (0..board_size * board_size)
                .into_iter()
                .map(map_fn)
                .collect()
        };

        results.into_iter().flatten().collect()
    }

    fn step_and_search(
        &self,
        loc: (usize, usize),
        seen: &HashSet<(i32, i32)>,
        found: &mut Vec<String>,
        cur: &Vec<char>,
    ) {
        // Extend the word by the current letter
        let mut new_cur = cur.clone();
        let chars = self.board.access((loc.1, loc.0)).to_char_vec();
        for c in chars {
            new_cur.push(c);
        }

        // If this is a valid word, put it into the seen word set
        if self.valid_word(&new_cur.iter().collect::<String>()) {
            found.push(new_cur.iter().collect::<String>());
        }

        if !self.has_extension(&new_cur.iter().collect::<String>()) {
            return;
        }

        // Update the seen dice set
        let mut new_seen = seen.clone();
        new_seen.insert((loc.0 as i32, loc.1 as i32));

        let steps = if self.diagonals {
            vec![
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
            ]
        } else {
            vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
        };

        for step in steps {
            // Take the step
            let new_pos = (loc.0 as i32 + step.0, loc.1 as i32 + step.1);

            // Not in bounds, skip
            if !self.board.in_bounds(&new_pos) {
                continue;
            }

            // Visited already
            if new_seen.contains(&new_pos) {
                continue;
            }

            self.step_and_search(
                (new_pos.0.try_into().unwrap(), new_pos.1.try_into().unwrap()),
                &new_seen,
                found,
                &new_cur,
            );
        }
    }

    fn valid_word(&self, word: &str) -> bool {
        word.len() > 2 && self.dictionary.check_word(word)
    }

    fn has_extension(&self, word: &str) -> bool {
        word.len() == 1 || self.dictionary.extend_word(&word).len() > 0
    }

    pub fn reshuffle(&mut self) {
        self.board = BoggleBoard::new(self.board_size);
        self.store_all_words();
    }
}

pub struct BoggleGame {
    boggle: BoggleSolver,
    found_words: HashSet<String>,
    game_time: i32,
}

impl BoggleGame {
    pub fn new(
        board_size: i32,
        game_time: i32,
        diagonals: bool,
        dictionary_path: String,
        multi_thread: bool,
    ) -> Self {
        Self {
            boggle: BoggleSolver::new(board_size, diagonals, dictionary_path.clone(), multi_thread),
            found_words: HashSet::new(),
            game_time,
        }
    }

    pub fn start(&mut self) {
        self.print_welcome_message();
        let (tx, rx) = mpsc::channel(); // Timer thread
        let timer_tx = tx.clone();
        let game_time = self.game_time;
        thread::spawn(move || {
            thread::sleep(Duration::new(game_time as u64, 0));
            timer_tx.send(String::from("TIME_UP")).unwrap();
        }); // Input listener thread

        let input_tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            let mut input = String::new();
            loop {
                input.clear();
                stdin.read_line(&mut input).expect("Failed to read line");
                let word = input.trim().to_uppercase();
                input_tx.send(word).unwrap();
            }
        }); // Main thread: wait for the timer or input

        loop {
            match rx.recv() {
                Ok(message) => {
                    if message == "TIME_UP" {
                        println!("\nTime's up!");
                        break;
                    } else {
                        self.process_word(&message.to_lowercase());
                    }
                }
                Err(_) => {
                    println!("Error receiving message.");
                    break;
                }
            }
        }
        self.print_final_scores();
    }

    fn print_welcome_message(&self) {
        println!("{}", self.boggle.board);
        println!(
            "Game started! Enter as many words as you can in {} seconds.",
            self.game_time
        );
    }

    fn process_word(&mut self, word: &str) {
        if self.boggle.get_possible_words().contains(word) {
            if self.found_words.contains(word) {
                println!("You have already found this word. Try again!");
            } else {
                self.found_words.insert(word.to_string());
            }
        } else {
            println!("Not a valid word. Try again!");
        }
    }

    fn print_final_scores(&self) {
        println!("\nFinal scores:\n");
        println!(
            "You found {} of a total {} possible words",
            self.found_words.len(),
            self.boggle.get_possible_words().len()
        );
        self.print_found_words();
        self.print_possible_words();
    }

    fn print_found_words(&self) {
        let mut found_word_vec: Vec<&String> = self.found_words.iter().collect();
        found_word_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut score = 0;
        for word in found_word_vec {
            score += word.len() - 2;
            println!("{} {}", word.to_uppercase(), word.len() - 2);
        }
        println!("\nYour final score: {}", score);
    }

    fn print_possible_words(&self) {
        let mut possible_word_vec: Vec<&String> = self
            .boggle
            .possible_words
            .difference(&self.found_words)
            .collect();
        possible_word_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        println!("\nYou could have found some of these words: ");
        for i in 0..15 {
            if i >= possible_word_vec.len() {
                continue;
            }
            println!(
                "{} {}",
                possible_word_vec[i].to_uppercase(),
                possible_word_vec[i].len() - 2
            );
        }
        let max_score: usize = possible_word_vec.iter().map(|word| word.len() - 2).sum();
        println!("\nYour potential max score: {}", max_score);
    }
}
