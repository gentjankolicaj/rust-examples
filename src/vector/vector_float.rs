use std::mem::{align_of_val, size_of_val};

fn main() {
    println!("float values in vector:");

    let v32: Vec<f32> = vec![1.0, 2.0, 3.0];
    let v64: Vec<f64> = vec![1.0, 2.0, 3.0];

    println!("v32: {:?}", v32);
    println!("v64: {:?}", v64);

    println!("size_of_val v32: {}", size_of_val(&v32));
    println!("align_of_val v32: {}", align_of_val(&v32));

    println!("size_of_val v64: {}", size_of_val(&v64));
    println!("align_of_val v64: {}", align_of_val(&v64));
}
