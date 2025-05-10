use std::mem::{align_of_val, size_of_val};

fn main() {
    let au: [(); 3] = [(), (), ()];

    println!("au={:?}", au);
    println!("au size_of_val={}", size_of_val(&au));
    println!("au align_of_val={}", align_of_val(&au));
}
