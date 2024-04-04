use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let arr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Invalid");
    let k: usize = k.trim().parse().expect("not valid");
    println!("{}", kth_small(arr, k))
}

fn kth_small(mut v:Vec<usize>, k: usize)-> usize {
    v.sort();
    v[k-1]

}
