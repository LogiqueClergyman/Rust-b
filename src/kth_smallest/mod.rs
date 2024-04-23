use std::io;
pub fn run(){
    println!("Enter the number of elements in the array");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading input");
    let n:u32 = n.trim().parse().expect("Invalid input. Please enter a positive number.");
    let mut arr:Vec<i32> = Vec::new();
    println!("Enter the elements:");
    for _ in 0..n{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Error reading input");
        let num:i32 = s.trim().parse().expect("Invalid input. Please enter a valid number.");
        arr.push(num);
    }
    arr.sort();
    println!("Enter the value of k");
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Error reading input");
    let k:u32 = k.trim().parse().expect("Invalid input. Please enter a positive number.");
    println!("The kth smallest element is: {}",arr[k as usize - 1]);
}