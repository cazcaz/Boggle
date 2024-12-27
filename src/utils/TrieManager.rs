use super::DictTrie::DictTrie;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const TRIE_FILE: &str = "english_trie.bin";
const JSON_FILE: &str = "simple_english_dictionary.json";

#[derive(Serialize, Deserialize)]
struct SerializableTrie {
    trie: DictTrie,
}

pub fn load_trie() -> Result<DictTrie, io::Error> {
    if Path::new(TRIE_FILE).exists() {
        let data = fs::read(TRIE_FILE)?;
        let serializable_trie: SerializableTrie = bincode::deserialize(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(serializable_trie.trie)
    } else {
        let trie = create_and_serialize_trie()?;
        Ok(trie)
    }
}

fn create_and_serialize_trie() -> Result<DictTrie, io::Error> {
    let words: Vec<String> = load_words_from_json(JSON_FILE)?;

    let mut trie = DictTrie::new();
    for word in words {
        trie.insert_word(&word.to_string());
    }

    let serializable_trie = SerializableTrie { trie: trie.clone() };
    let serialized_data = bincode::serialize(&serializable_trie)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut file = fs::File::create(TRIE_FILE)?;
    file.write_all(&serialized_data)?;

    Ok(trie)
}

fn load_words_from_json(file_path: &str) -> Result<Vec<String>, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let json_value: Value =
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut words = Vec::new();
    if let Value::Object(map) = json_value {
        for (word, _desc) in map {
            if word.chars().all(|c| c.is_ascii_alphabetic()) {
                words.push(word.to_lowercase());
            }
        }
    }
    Ok(words)
}
