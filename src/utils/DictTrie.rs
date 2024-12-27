use std::collections::{HashMap, HashSet};

struct DictTrieNode {
    children: HashMap<char, DictTrieNode>,
    end: bool,
}

impl DictTrieNode {
    fn new(c: char) -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }
}

pub struct DictTrie {
    root: DictTrieNode,
}

impl DictTrie {
    fn new() -> Self {
        Self {
            root: DictTrieNode::new('0'),
        }
    }

    fn insert_word(&mut self, word: &String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert_with(|| DictTrieNode::new(c));
        }
        current_node.end = true;
    }

    fn check_word(&self, word: &String) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(child) => current_node = child,
                None => return false,
            }
        }
        current_node.end
    }

    fn extend_word(&self, word: &String) -> Vec<String> {
        let mut results = Vec::new();
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(child) => current_node = child,
                None => return results,
            }
        }

        fn collect_words(node: &DictTrieNode, prefix: &str, results: &mut Vec<String>) {
            let mut current_word = prefix.to_string();
            if node.end {
                results.push(current_word.clone());
            }
            for (ch, child) in &node.children {
                current_word.push(*ch);
                collect_words(child, &current_word, results);
                current_word.pop();
            }
        }

        collect_words(current_node, word, &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_words() {
        let test_words: Vec<String> = vec!["Test", "Testing", "Taught", "Dog", "Door", "Dot"]
            .iter()
            .map(|&s| String::from(s))
            .collect();
        let mut trie = DictTrie::new();
        for word in test_words {
            trie.insert_word(&word);
        }

        assert_eq!(trie.check_word(&String::from("Test")), true);
        assert_eq!(trie.check_word(&String::from("Testing")), true);
        assert_eq!(trie.check_word(&String::from("Dog")), true);
        assert_eq!(trie.check_word(&String::from("Do")), false);
        assert_eq!(trie.check_word(&String::from("Test")), true);
        assert_eq!(trie.check_word(&String::from("Dogs")), false);
        assert_eq!(trie.check_word(&String::from("")), false);
    }

    #[test]
    fn test_extend_word() {
        let test_words: Vec<String> = vec!["Test", "Testing", "Taught", "Dog", "Door", "Dot"]
            .iter()
            .map(|&s| String::from(s))
            .collect();
        let mut trie = DictTrie::new();
        for word in test_words {
            trie.insert_word(&word);
        }

        let extensions: HashSet<String> =
            trie.extend_word(&String::from("Te")).into_iter().collect();
        let expected: HashSet<String> = vec![String::from("Test"), String::from("Testing")]
            .into_iter()
            .collect();

        assert_eq!(extensions, expected);

        let extensions: HashSet<String> =
            trie.extend_word(&String::from("Do")).into_iter().collect();
        let expected: HashSet<String> = vec![
            String::from("Dog"),
            String::from("Door"),
            String::from("Dot"),
        ]
        .into_iter()
        .collect();

        assert_eq!(extensions, expected);

        let extensions: HashSet<String> =
            trie.extend_word(&String::from("Ta")).into_iter().collect();
        let expected: HashSet<String> = vec![String::from("Taught")].into_iter().collect();

        assert_eq!(extensions, expected);
    }
}