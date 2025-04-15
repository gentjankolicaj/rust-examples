fn main() {
    let f32_value = 3.14f32;

    match f32_value {
        i if i < 3.0 => println!("i is less than 3.0!"),
        i if i > 3.0 => println!("i is more than 3.0!"),
        _ => println!("i is 0!"),
    }
}
