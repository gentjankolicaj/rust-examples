use std::mem::{align_of_val, size_of_val};
fn main() {
    println!("signed integer values in vectors: ");
    let v8: Vec<i8> = vec![1, 2, 3];
    let v16: Vec<i16> = vec![1i16, 2, 3];
    let v32: Vec<i32> = vec![1i32, 2, 3];
    let v64: Vec<i64> = vec![1i64, 2, 3];
    let v128: Vec<i128> = vec![1i128, 2, 3];

    println!("v8: {:?}", v8);
    println!("v16: {:?}", v16);
    println!("v32: {:?}", v32);
    println!("v64: {:?}", v64);
    println!("v128: {:?}", v128);

    println!("size_of_val v8: {}", size_of_val(&v8));
    println!("align_of_val v8: {}", align_of_val(&v8));

    println!("size_of_val v16: {}", size_of_val(&v16));
    println!("align_of_val v16: {}", align_of_val(&v16));

    println!("size_of_val v32: {}", size_of_val(&v32));
    println!("align_of_val v32: {}", align_of_val(&v32));

    println!("size_of_val v64: {}", size_of_val(&v64));
    println!("align_of_val v64: {}", align_of_val(&v64));

    println!("size_of_val v128: {}", size_of_val(&v128));
    println!("align_of_val v128: {}", align_of_val(&v128));
}
