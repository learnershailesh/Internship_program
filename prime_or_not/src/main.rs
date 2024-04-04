use std::io;
fn main() {
    loop{
        println!("Please input the number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        let  ans: bool = prime_num(num);
    
    if ans == true {
        println!("{} is not a prime number", num );
    } else {
        println!("{} is a prime number", num);
    }
} 
}

fn prime_num(number: i32)-> bool{
       
        if number < 2 {
            return false;        
        }
        if number ==2 || number == 3 || number == 5 {
            return true;
        }
         else if number % 2 ==  0 || number % 3 == 0 || number % 5 == 0 {
             return  false;
        } else {
            return true;
        }
    }

