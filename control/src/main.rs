use std::io;
fn main() {
    println!("enter your number");
    let mut m=String::new();
    io::stdin().read_line(&mut m)
        .expect("Failed to read line");
    let m:u32=m.trim().parse()
        .expect("Failed to prase");
    if m>20{
        println!("you win");
    }else{
        println!("you lose");
    }
}
