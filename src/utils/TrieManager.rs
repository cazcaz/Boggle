use super::DictTrie::DictTrie;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const TRIE_FILE: &str = "english_trie.bin";

#[derive(Serialize, Deserialize)]
pub struct SerializableTrie {
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
    // Load the dictionary words from a file or a hardcoded list
    let words: Vec<String> = vec![];

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
