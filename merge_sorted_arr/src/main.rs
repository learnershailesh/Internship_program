use std::io;
fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("something went wrong");
    let  mut arr1: Vec<usize> = input1.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("something went wrong");
    let  mut arr2: Vec<usize> = input2.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    arr1.sort();
    arr2.sort();
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    let  final_rr = [arr1, arr2].concat();
    println!("{:?}", final_rr);

}
