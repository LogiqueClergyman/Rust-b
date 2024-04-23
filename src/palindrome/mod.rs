use std::io;

pub fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let rev: String = s.chars().rev().collect();

    return match s == rev {
        true => true,
        false => false,
    };
}

pub fn run() {
    let mut word = String::new();
    println!("Enter the word to check for palindrome: ");
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();
    match is_palindrome(word) {
        true => println!("{} is a palindrome", word),
        false => println!("{} is not a palindrome", word),
    };
    return;
}
