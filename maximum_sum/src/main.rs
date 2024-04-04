use std::io;
fn main() {
    println!("enter the array");
    let mut input = String:: new();
    io::stdin().read_line(&mut input).expect("Not valid ");
    let arr: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!(" maximum subarray sum: {}", maxim_sum(arr));
}

fn maxim_sum(arr: Vec<i64>)-> i64 {
    let mut max_sum = arr[0];
    let mut curr_sum  = 0;

    for i in 0..arr.len() {
        curr_sum+=arr[i];

        if curr_sum > max_sum {
            max_sum = curr_sum;
        }

        if curr_sum < 0 {
            curr_sum = 0;
        }
    }

    max_sum
}
