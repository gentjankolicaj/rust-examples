use std::mem::{align_of_val, size_of_val};

fn main() {
    let ab: [bool; 4] = [true, false, false, true];

    println!("ab={:?}", ab);
    println!("ab size_of_val={}", size_of_val(&ab));
    println!("ab align_of_val={}", align_of_val(&ab));
}
