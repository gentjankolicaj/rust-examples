use std::mem::{align_of_val, size_of_val};

fn main() {
    let ac: [char; 3] = ['A', 'B', 'C'];

    println!("ac: {:?}", ac);
    println!("ac size_of_val={}", size_of_val(&ac));
    println!("ac align_of_val={}", align_of_val(&ac));
}
