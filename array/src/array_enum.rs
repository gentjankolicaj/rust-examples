use std::mem::{align_of_val, size_of_val};

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

fn main() {
    let ac: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];

    println!("ac={:?}", ac);
    println!("ac size_of_val={}", size_of_val(&ac));
    println!("ac align_of_val={}", align_of_val(&ac));
}
