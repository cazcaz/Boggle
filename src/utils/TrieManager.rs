use super::DictTrie::DictTrie;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const TRIE_FILE: &str = "english_trie.bin";
const JSON_FILE: &str = "dictionary.json";

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
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let vec: Vec<String> = serde_json::from_reader(reader)?;
    Ok(vec)
}
