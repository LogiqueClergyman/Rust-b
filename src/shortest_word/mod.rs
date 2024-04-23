use std::io;
pub fn run(){
    println!("Enter the string of words: ");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Error reading input");
    let words: Vec<&str> = input_string.split_whitespace().collect();
    let mut shortest_word = words[0];
    for word in words.iter(){
        if word.len() < shortest_word.len(){
            shortest_word = word;
        }
    }
    println!("The shortest word is: {}", shortest_word);
    return;
}