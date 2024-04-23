use std::io;
pub fn run() {
    println!("Enter the length of the first array:");
    let mut l1 = String::new();
    io::stdin().read_line(&mut l1).expect("Error reading input");
    let l1: usize = l1.trim().parse().expect("Invalid input");
    println!("Enter the elements of the first array:");
    let mut arr1: Vec<i32> = Vec::new();
    for _ in 0..l1 {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error reading input");
        let num: i32 = num.trim().parse().expect("Invalid input");
        arr1.push(num);
    }
    println!("Enter the length of the second array:");
    let mut l2 = String::new();
    io::stdin().read_line(&mut l2).expect("Error reading input");
    let l2: usize = l2.trim().parse().expect("Invalid input");
    println!("Enter the elements of the first array:");
    let mut arr2: Vec<i32> = Vec::new();
    for _ in 0..l2 {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error reading input");
        let num: i32 = num.trim().parse().expect("Invalid input");
        arr2.push(num);
    }
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..l1 {
        ans.push(arr1[i]);
    }
    for i in 0..l2 {
        ans.push(arr2[i]);
    }
    ans.sort();
    println!("Merged array: {:?}", ans);
}
