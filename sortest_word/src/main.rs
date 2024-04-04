use std::io;
fn main() {
    let mut randstr = String::new();
    io::stdin().read_line(&mut randstr).expect("somthing went wrong");
    let ans = randstr.split_whitespace().min_by_key(|word|word.len());
     println!("{:?}",ans);
}
