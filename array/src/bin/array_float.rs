use std::mem::{align_of_val, size_of_val};

fn main() {
    let af32: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    let af64: [f64; 4] = [1.0, 2.0, 3.0, 4.0];

    println!("af32:{:?}", af32);
    println!("af64:{:?}", af64);

    println!("af32 size_of_val in array:{}", size_of_val(&af32));
    println!("af32 align_of_val in array:{}", align_of_val(&af32));

    println!("af64 size_of_val in array:{}", size_of_val(&af64));
    println!("af64 align_of_val in array:{}", align_of_val(&af64));
}
