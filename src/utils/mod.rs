use std::io::{self, Write};

mod DictTrie;

mod TrieManager;

pub fn trie_CLI() {
    let mut trie = match TrieManager::load_trie() {
        Ok(trie) => {
            println!("Trie loaded successfully. Ready for lookups.");
            trie
        }
        Err(e) => {
            eprintln!("Failed to load or create the Trie: {}", e);
            return;
        }
    };

    loop {
        println!("\nEnter a command:");
        println!("1: Insert word");
        println!("2: Check word");
        println!("3: Extend word");
        println!("4: Exit");

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => insert_word(&mut trie),
            "2" => check_word(&trie),
            "3" => extend_word(&trie),
            "4" => break,
            _ => println!("Invalid command. Please enter 1, 2, 3, or 4."),
        }
    }
}

fn insert_word(trie: &mut DictTrie::DictTrie) {
    println!("Enter a word to insert:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim().to_string();

    trie.insert_word(&word);
    println!("Word '{}' inserted into the Trie.", word);
}

fn check_word(trie: &DictTrie::DictTrie) {
    println!("Enter a word to check:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim().to_string();

    if trie.check_word(&word) {
        println!("The word '{}' exists in the Trie.", word);
    } else {
        println!("The word '{}' does not exist in the Trie.", word);
    }
}

fn extend_word(trie: &DictTrie::DictTrie) {
    println!("Enter a word to find extensions:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim().to_string();

    let extensions = trie.extend_word(&word);
    if extensions.is_empty() {
        println!("No extensions found for the word '{}'.", word);
    } else {
        println!("Extensions for the word '{}': {:?}", word, extensions);
    }
}
