use std::io;
pub fn run() {
    println!("Enter a string to reverse:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Error reading input");
    let reversed = word.trim().chars().rev().collect::<String>();
    println!("Reversed string: {}", reversed);
}
