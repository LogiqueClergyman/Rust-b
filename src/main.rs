mod first_occurence;
mod is_prime;
mod kth_smallest;
mod longest_common_prefix;
mod max_depth;
mod max_subarray_sum;
mod median;
mod merge_sorted_arrays;
mod palindrome;
mod reverse_string;
mod shortest_word;

use std::io;

fn main() {
    println!("Choose a program to run:");
    println!("1. Check if a string is a palindrome");
    println!("2. Find the index of the first occurrence of a number in a sorted array");
    println!("3. Find the shortest word in a string of words");
    println!("4. Check if a number is prime");
    println!("5. Find the median of a sorted array");
    println!("6. Find the longest common prefix of a set of strings");
    println!("7. Find the kth smallest element in an array");
    println!("8. Find the maximum depth of a binary tree");
    println!("9. Reverse a string");
    println!("10. Check if a number is prime");
    println!("11. Merge two sorted arrays");
    println!("12. Find the maximum subarray sum");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    match choice {
        1 => palindrome::run(),
        2 => first_occurence::run(),
        3 => shortest_word::run(),
        4 => is_prime::run(),
        5 => median::run(),
        6 => longest_common_prefix::run(),
        7 => kth_smallest::run(),
        8 => max_depth::run(),
        9 => reverse_string::run(),
        10 => is_prime::run(),
        11 => merge_sorted_arrays::run(),
        12 => max_subarray_sum::run(),
        _ => println!("Invalid choice. Please choose a number between 1 and 12."),
    }
}
