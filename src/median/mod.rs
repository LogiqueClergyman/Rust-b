use std::io;
pub fn run(){
    println!("Enter the number of elements in the array");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading input");
    let n:u32 = n.trim().parse().expect("Invalid input. Please enter a positive number.");
    let mut arr:Vec<u32> = Vec::new();
    for _ in 0..n{
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Error reading input");
        let num:u32 = num.trim().parse().expect("Invalid input. Please enter a positive number.");
        arr.push(num);
    }
    if(arr.len()%2==0){
        println!("The median of the array is {}",(arr[arr.len()/2]+arr[arr.len()/2-1])/2);
        return;
    }
    else{
        println!("The median of the array is {}",arr[arr.len()/2]);
        return;
    }
}