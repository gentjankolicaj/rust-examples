use std::mem::{align_of_val, size_of_val};

fn main() {
    println!("char values in vector:");

    let vc: Vec<char> = vec!['A', 'B', 'C', 'D', 'E'];

    println!("vc: {:?}", vc);
    println!("size_of_val vc: {}", size_of_val(&vc));
    println!("align_of_val vc: {}", align_of_val(&vc));
}
