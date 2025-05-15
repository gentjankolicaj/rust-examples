fn main() {
    //declare variable with signed integer 2^8
    // range 0...2^8-1 else compile error
    // 0...255
    let a_8bit: u8 = 255;

    //declare variable with signed integer 2^16
    // range 0...2^16-1 else compile error
    let a_16bit: u16 = 1 << 15;

    //declare variable with signed integer 2^32
    // range 0...2^32-1 else compile error
    let a_32bit: u32 = 1 << 31;

    //declare variable with signed integer 2^64
    //range 0...2^64-1 else compile error
    let a_64bit: u64 = 1 << 63;

    //declare variable with signed integer 2^128
    //range 0...2^128-1 else compile error
    let a_128bit: u128 = 1 << 127;

    let a_pointer: usize = 10;

    println!("a_8bit={}", a_8bit);
    println!("a_16bit={}", a_16bit);
    println!("a_32bit={}", a_32bit);
    println!("a_64bit={}", a_64bit);
    println!("a_128bit={}", a_128bit);
    println!("a_pointer={}", a_pointer);
}
