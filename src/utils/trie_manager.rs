use super::dict_trie::DictTrie;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const TRIE_FILE: &str = "english_trie.bin";

#[derive(Serialize, Deserialize)]
struct SerializableTrie {
    trie: DictTrie,
}

pub fn load_trie(dictionary_path: String) -> Result<DictTrie, io::Error> {
    let exe_dir = std::env::current_exe()?
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let file_path: PathBuf = exe_dir.join("resources").join(dictionary_path);
    if Path::new(TRIE_FILE).exists() {
        let data = fs::read(TRIE_FILE)?;
        let serializable_trie: SerializableTrie = bincode::deserialize(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(serializable_trie.trie)
    } else {
        let trie = create_and_serialize_trie(file_path)?;
        Ok(trie)
    }
}

fn create_and_serialize_trie(dictionary_path: PathBuf) -> Result<DictTrie, io::Error> {
    let words: Vec<String> = load_words_from_json(dictionary_path)?;

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

fn load_words_from_json(file_path: PathBuf) -> Result<Vec<String>, io::Error> {
    dbg!(&file_path);
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let vec: Vec<String> = serde_json::from_reader(reader)?;
    Ok(vec)
}
