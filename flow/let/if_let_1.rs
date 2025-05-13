//1.In rust-lang 'if-let' is used in place of 'match' sometimes.
//2.This means that 'if let' is used for destructing
//3.In rust-lang 'if-let' can be used for destructing enums.

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

fn main() {
    let r = Color::RED;
    let b = Color::BLUE;

    //We read it as : if let destructures r into Color::RED
    if let Color::RED = r {
        println!("r variable is destructure into: {:?}", Color::RED);
    }

    //We read it as : if let destructures b into Color::BLUE
    if let Color::BLUE = b {
        println!("b variable is destructure into: {:?}", Color::BLUE);
    }
}
