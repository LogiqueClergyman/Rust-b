use std::collections::HashMap;
use std::io;
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode {
                children: HashMap::new(),
                is_end: false,
            });
        }
        node.is_end = true;
    }

    fn longest_common_prefix(&self) -> String {
        let mut prefix = String::new();
        let mut node = self;
        while node.children.len() == 1 && !node.is_end {
            let (&ch, next_node) = node.children.iter().next().unwrap();
            prefix.push(ch);
            node = next_node;
        }
        return prefix;
    }
}

pub fn run() {
    println!("Enter the number of strings in the array");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading input");
    let n: u32 = n
        .trim()
        .parse()
        .expect("Invalid input. Please enter a positive number.");
    let mut trie = TrieNode {
        children: HashMap::new(),
        is_end: false,
    };
    println!("Enter the strings:");
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Error reading input");
        trie.insert(s.trim());
    }

    let longest_common_prefix = trie.longest_common_prefix();
    match longest_common_prefix.len() {
        0 => println!("No common prefix found"),
        _ => println!("Longest common prefix: {}", longest_common_prefix),
        
    }
}
