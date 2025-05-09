use std::mem::{align_of_val, size_of_val};
fn main() {
    println!("unsigned integer values in arrays: ");
    let a8: [u8; 3] = [1, 2, 3];
    let a16: [u16; 3] = [1, 2, 3];
    let a32: [u32; 3] = [1, 2, 3];
    let a64: [u64; 3] = [1, 2, 3];
    let a128: [u128; 3] = [1, 2, 3];

    println!("a8: {:?}", a8);
    println!("a16: {:?}", a16);
    println!("a32: {:?}", a32);
    println!("a64: {:?}", a64);
    println!("a128: {:?}", a128);

    println!("size_of_val a8: {} bytes", size_of_val(&a8));
    println!("align_of_val a8: {} bytes", align_of_val(&a8));

    println!("size_of_val a16: {} bytes", size_of_val(&a16));
    println!("align_of_val a16: {} bytes", align_of_val(&a16));

    println!("size_of_val a32: {} bytes", size_of_val(&a32));
    println!("align_of_val a32: {} bytes", align_of_val(&a32));

    println!("size_of_val a64: {} bytes", size_of_val(&a64));
    println!("align_of_val a64: {} bytes", align_of_val(&a64));

    println!("size_of_val a128: {} bytes", size_of_val(&a128));
    println!("align_of_val a128: {} bytes", align_of_val(&a128));
}
