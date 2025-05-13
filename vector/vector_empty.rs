use std::mem::{align_of_val, size_of_val};

fn main() {
    println!("empty values in vector:");

    let ve: Vec<()> = vec![(), (), ()];

    println!("ve: {:?}", ve);
    println!("size_of_val ve: {}", size_of_val(&ve));
    println!("align_of_val ve: {}", align_of_val(&ve));
}
