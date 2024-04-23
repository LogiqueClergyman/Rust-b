use std::io;
pub fn run(){
    println!("Enter the array elemets seperated by whitespace: ");
    let mut input_array = String::new();
    io::stdin().read_line(&mut input_array).expect("Error reading input");
    let array: Vec<String> = input_array.split_whitespace().map(|s| s.to_string()).collect();
    let mut nums: Vec<i32> = Vec::new();
    for num in array.iter(){
        nums.push(num.parse().expect("Error parsing input"));
    }
    println!("Enter the number to find: ");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Error reading input");
    let target: i32 = target.trim().parse().expect("Error parsing input");
    let mut index = 0;
    for num in nums.iter(){
        if num == &target{
            println!("The first occurence of {} is at index {}", target, index);
            return;
        }
        index += 1;
    }
    println!("The number {} is not in the array", target);
    return;
}