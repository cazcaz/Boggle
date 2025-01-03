use boggle_utils::boggle_board::BoggleBoard;
use boggle_utils::boggle_char::BoggleChar;
use rayon::prelude::*;
use std::collections::HashSet;
use std::io;
use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;
use utils::dict_trie::DictTrieNode;

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

    // To solve from a custom board
    pub fn from(
        board: Vec<char>,
        board_size: i32,
        diagonals: bool,
        dictionary_path: String,
        multi_thread: bool,
    ) -> Self {
        Self {
            board: BoggleBoard::from(board, board_size),
            possible_words: HashSet::new(),
            board_size,
            diagonals,
            dictionary: utils::trie_manager::load_trie(dictionary_path)
                .unwrap_or_else(|e| panic!("Failed to load or create the Trie: {}", e)),
            multi_thread,
        }
    }

    pub fn get_board(&self) -> BoggleBoard {
        self.board.clone()
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
            let mut result = Vec::new();
            self.step_and_search(
                (x, y),
                &mut HashSet::<(i32, i32)>::new(),
                &mut result,
                &mut vec![],
                self.dictionary.get_start_node(),
            );
            result
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
        loc: (i32, i32),
        seen: &mut HashSet<(i32, i32)>,
        found: &mut Vec<String>,
        cur_word: &mut Vec<char>,
        prev_node: &DictTrieNode,
    ) {
        let mut cur_node: &DictTrieNode = prev_node;
        // If we have stepped here but it was the wrong choice, return early
        if seen.contains(&loc) {
            return;
        }

        let current_letter: BoggleChar = self.board.access((loc.1 as usize, loc.0 as usize));
        match current_letter {
            BoggleChar::Qu => {
                if let Some(node) = cur_node.get_child(&'q') {
                    cur_node = node;
                } else {
                    return;
                }
                cur_word.push('q');
                if let Some(node) = cur_node.get_child(&'u') {
                    cur_node = node;
                } else {
                    cur_word.pop();
                    return;
                }
                cur_word.push('u');
            }
            _ => {
                let letter = current_letter.to_char_vec()[0];
                if let Some(node) = cur_node.get_child(&letter) {
                    cur_node = node;
                } else {
                    return;
                }
                cur_word.push(letter);
            }
        }

        // Check if the current location is valid
        seen.insert(loc);

        // If this is a valid word, put it into the seen word set
        if cur_word.len() > 2 && cur_node.end {
            found.push(cur_word.iter().collect::<String>());
        }

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
            let new_pos = (loc.0 + step.0, loc.1 + step.1);

            // Not in bounds, skip
            if !self.board.in_bounds(&new_pos) {
                continue;
            }

            self.step_and_search(new_pos, seen, found, cur_word, &cur_node);
        }

        seen.remove(&loc);

        // Special handling for Qu
        match current_letter {
            BoggleChar::Qu => {
                cur_word.pop();
                cur_word.pop();
            }
            _ => {
                cur_word.pop();
            }
        }
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

pub struct BoggleSolverInterface {
    pub boggle: BoggleSolver,
}

impl BoggleSolverInterface {
    pub fn new(
        board: String,
        board_size: i32,
        diagonals: bool,
        dictionary_path: String,
        multi_thread: bool,
    ) -> Self {
        if board.len() == 0 {
            let mut solver: Self = Self {
                boggle: BoggleSolver::new(board_size, diagonals, dictionary_path, multi_thread),
            };
            solver.boggle.store_all_words();
            return solver;
        }
        if board.len() != board_size as usize * board_size as usize {
            panic!("Board size was not correct!");
        }
        let mut solver: Self = Self {
            boggle: BoggleSolver::from(
                board.chars().into_iter().collect(),
                board_size,
                diagonals,
                dictionary_path,
                multi_thread,
            ),
        };
        solver.boggle.store_all_words();
        solver
    }

    pub fn output_words(&self) {
        let mut possible_word_vec: Vec<String> =
            self.boggle.get_possible_words().into_iter().collect();
        possible_word_vec.sort_by(|a, b| {
            let len_cmp = b.len().cmp(&a.len());
            if len_cmp == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                len_cmp
            }
        });
        for word in &possible_word_vec {
            println!("{} {}", word, word.len());
        }
        println!("\n{}", possible_word_vec.len());
    }
}
