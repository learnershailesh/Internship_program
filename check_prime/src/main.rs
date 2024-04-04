use std::io;
fn main() {
    let  mut input = String::new();
    io::stdin().read_line(&mut input).expect("failled to read");
    let num: i32 = input.trim().parse().expect("Invalid");
    if num<2 {
        println!("{} : not a prime_number", num);
    }
    if num == 2 || num == 3 || num ==  5 {
        println!("{} : is a prime number", num);
    }
     else if num % 2 == 0 || num % 3 == 0 ||  num % 5 == 0 {
        println!("{} not a prime number", num)
    } else {
        println!("{} a prime number", num);
    }
}
