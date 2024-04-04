use std::io;
fn main() {
    let  mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid");
    let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("not valid");
    let k: i32 = k.trim().parse().expect("Invalid");

    println!("{:?}", first_occ(&arr, k))
}

fn first_occ(arr: &[i32], target: i32)-> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i);
        }
    }
    None
}
