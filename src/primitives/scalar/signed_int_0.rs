fn main() {
    //declare variable with signed integer 2^8
    // range -2^7...2^7-1 else compile error
    // let a:i8 =200; compile error
    let a_8bit: i8 = 127; //max positive value
    let b_8bit: i8 = -128; //max negative value

    //declare variable with signed integer 2^16
    // range -2^15...2^15-1 else compile error
    let a_16bit: i16 = 1024;

    //declare variable with signed integer 2^32
    // range -2^31...2^31-1 else compile error
    let a_32bit: i32 = 1024 * 1024;

    //declare variable with signed integer 2^64
    // range -2^63...2^63-1 else compile error
    let a_64bit: i64 = 1024 * 1024 * 1024;

    //declare variable with signed integer 2^128
    // range -2^127...2^127-1 else compile error
    let a_128bit: i128 = 1024 * 1024 * 1024;

    let a_pointer: isize = 10;

    println!("a_8bit={}", a_8bit);
    println!("b_8bit={}", b_8bit);
    println!("a_16bit={}", a_16bit);
    println!("a_32bit={}", a_32bit);
    println!("a_64bit={}", a_64bit);
    println!("a_128bit={}", a_128bit);
    println!("a_pointer={}", a_pointer);
}
