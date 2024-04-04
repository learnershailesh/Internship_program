use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid");
    let  arr: Vec<u32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", median_array(arr));

}
fn median_array(v: Vec<u32>) -> u32{
    let mut mid: usize = 0;
    let n: usize = v.len();
    if n % 2 !=0 {
         mid += (n)/2;
         v[mid]
    }else {
         mid += ((n)/2 + ((n/2)+1))/2;
         v[mid-1]
    }
   
}
