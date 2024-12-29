use boggle_utils::boggle_board::BoggleBoard;
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
}

impl BoggleSolver {
    pub fn new(board_size: i32, diagonals: bool, dictionary_path: String) -> Self {
        let mut boggle_board = Self {
            board: BoggleBoard::new(board_size),
            possible_words: HashSet::new(),
            board_size,
            diagonals,
            dictionary: utils::trie_manager::load_trie(dictionary_path)
                .unwrap_or_else(|e| panic!("Failed to load or create the Trie: {}", e)),
        };
        boggle_board.find_all_words();
        boggle_board
    }

    pub fn get_possible_words(&self) -> HashSet<String> {
        self.possible_words.clone()
    }

    fn find_all_words(&mut self) -> Vec<String> {
        let result: Vec<String> = vec![];

        for y in 0..self.board_size {
            for x in 0..self.board_size {
                let mut starting_letter = String::new();
                self.board
                    .access((y as usize, x as usize))
                    .append_to(&mut starting_letter);
                if self.has_extension(&starting_letter) {
                    let seen_indices = HashSet::<(i32, i32)>::new();
                    let empty = String::new();
                    self.step_and_search((x as usize, y as usize), &seen_indices, &empty);
                }
            }
        }
        result
    }

    fn step_and_search(&mut self, loc: (usize, usize), seen: &HashSet<(i32, i32)>, cur: &String) {
        // Extend the word by the current letter
        let mut new_cur = cur.clone();
        self.board.access((loc.1, loc.0)).append_to(&mut new_cur);

        // If this is a valid word, put it into the seen word set
        if self.valid_word(&new_cur) {
            self.possible_words.insert(new_cur.clone());
        }

        if !self.has_extension(&new_cur) {
            return;
        }

        // Update the seen dice set
        let mut new_seen = seen.clone();
        new_seen.insert((loc.0 as i32, loc.1 as i32));

        // Every direction to step in
        let mut steps: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        if self.diagonals {
            // Add in diagonal steps
            steps.push((1, 1));
            steps.push((-1, 1));
            steps.push((1, -1));
            steps.push((-1, -1));
        }

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
                &new_cur,
            );
        }
    }

    fn valid_word(&self, word: &String) -> bool {
        word.len() > 2 && self.dictionary.check_word(word)
    }

    fn has_extension(&self, word: &String) -> bool {
        word.len() == 1 || self.dictionary.extend_word(&word).len() > 0
    }

    pub fn reshuffle(&mut self) {
        self.board = BoggleBoard::new(self.board_size);
        self.possible_words = HashSet::new();
        self.find_all_words();
    }
}

pub struct BoggleGame {
    boggle: BoggleSolver,
    found_words: HashSet<String>,
    game_time: i32,
}

impl BoggleGame {
    pub fn new(board_size: i32, game_time: i32, diagonals: bool, dictionary_path: String) -> Self {
        Self {
            boggle: BoggleSolver::new(board_size, diagonals, dictionary_path.clone()),
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
                        self.process_word(&message);
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
            println!("{} {}", word, word.len() - 2);
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
                possible_word_vec[i],
                possible_word_vec[i].len() - 2
            );
        }
        let max_score: usize = possible_word_vec.iter().map(|word| word.len() - 2).sum();
        println!("\nYour potential max score: {}", max_score);
    }
}
