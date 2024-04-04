use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("invalid");
    palindrone_str(&input);

}

fn palindrone_str(str1: &str) -> bool {
    let  revstr  = str1.chars().rev().collect::<String>();
   // println!("{}", revstr);
     if str1 == revstr {
         println!("{} is a palindrone", str1);
         return  true;
     } else {
         println!("{} is not a palindrone", str1);
         return  false;
     }
}
