fn main() {
    let i32_value = 10;

    match i32_value {
        i if i < 0 => println!("i is negative!"),
        i if i > 0 => println!("i is positive!"),
        _ => println!("i is 0!"),
    }
}
