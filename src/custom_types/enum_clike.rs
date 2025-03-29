//1.Using enums as c-enum like.

#[derive(Debug)]
enum Number {
    ZERO,
    ONE,
    TWO,
}
#[derive(Debug)]
enum Color {
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}

//Alias at Number enum
type Int = Number;
fn main() {
    //create variable with value of type enum-variant from alias & enum.
    let zero = Int::ZERO;
    let red = Color::RED;
    println!("{:?}", zero);
    println!("{:?}", red);
}

