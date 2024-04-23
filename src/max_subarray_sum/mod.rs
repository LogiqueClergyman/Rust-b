use std::io;
pub fn run(){
    println!("Enter the array of numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().expect("Please enter valid integers"))
        .collect();
    let result = max_subarray_sum(nums);
    println!("The maximum subarray sum is: {}", result);
}
fn max_subarray_sum(nums: Vec<i32>) -> i32 {
    let mut i = 0; 
    let mut j = 1;
    let mut sum = nums[0];
    let mut max = nums[0];
    while j < nums.len(){
        sum += nums[j];
        if sum > max {
            max = sum;
        }
        if sum < 0 {
            sum = 0;
            i = j+1;
        }
        j += 1;
    }
    return max;
}