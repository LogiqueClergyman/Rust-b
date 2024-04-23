use std::io;
pub fn run(){
    println!("Enter the number to check for prime: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error reading input");
    let num:u32 = num.trim().parse().expect("Invalid input. Please enter a positive number.");
    if num == 0 || num == 1{
        println!("{} is not a prime number", num);
        return;
    }
    for i in 2..=num/2{
        if num % i == 0{
            println!("{} is not a prime number", num);
            return;
        }
    }
    println!("{} is a prime number", num);
    return;
}