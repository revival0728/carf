use std::collections::BTreeMap;

type Map = BTreeMap<char, TrieNode>;

struct TrieNode {
  pub children: Map,
  pub value: char,
  pub is_end: bool,
}

impl TrieNode {
  pub fn new(value: char) -> TrieNode {
    TrieNode {
      value,
      children: Map::new(),
      is_end: false,
    }
  }
}
impl Default for TrieNode {
  fn default() -> TrieNode {
    TrieNode {
      value: '\0',
      children: Map::new(),
      is_end: false,
    }
  }
}

pub struct Trie {
  root: TrieNode,
}

impl Trie {
  pub fn new() -> Trie {
    Trie {
      root: TrieNode::default(),
    }
  }
  pub fn insert(&mut self, word: &str) {
    let mut current = &mut self.root;
    for c in word.chars() {
      current = current.children.entry(c).or_insert(TrieNode::new(c));
    }
    current.is_end = true;
  }
  pub fn contains(&self, word: &str) -> bool {
    let mut current = &self.root;
    for c in word.chars() {
      match current.children.get(&c) {
        Some(node) => current = node,
        None => return false,
      }
    }
    current.is_end
  }
}