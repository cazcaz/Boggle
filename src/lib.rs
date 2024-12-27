use rand::Rng;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

pub mod utils;

#[derive(Clone)]
pub struct Dice {
    array: Vec<Vec<char>>,
    possible_words: HashSet<String>,
}

impl Dice {
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
        Self {
            array: dice,
            possible_words: HashSet::new(),
        }
    }

    pub fn get_possible_words(&self) -> HashSet<String> {
        self.possible_words.clone()
    }

    pub fn find_all_words(&mut self) -> Vec<String> {
        let result: Vec<String> = vec![];
        let mut dictionary = match utils::TrieManager::load_trie() {
            Ok(dictionary) => dictionary,
            Err(e) => {
                panic!("Failed to load or create the Trie: {}", e);
            }
        };

        // Navigate around the board and record which have been visited
        // Append the next letter to the current word
        // If the Trie says there exists words that start with the current string, then continue
        // If the current word exists add it to the list of words.

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

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.array {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
