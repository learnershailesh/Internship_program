use std::io;
fn main() {
    println!("please enter the string");
    let  mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("failled to read");
    println!("your input string: {}", str1);

    println!("{}", str1.chars().rev().collect::<String>());
    
}
