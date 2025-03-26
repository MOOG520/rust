fn main() {
    println!("Hello, world!");
    another_function();
    let x='a';
    let y:char=out_word(x);
    println!("y is {}",y);
}

fn out_word(x:char) -> char{
    x+'1'
}
fn another_function(){
    println!("I want to change my way of life");

}
