//1.Casting is explicit type conversion of primitives in rust.
//2.rust-lang does not provide implicit type casting (coercion) between primitives.
//3.CASTING BNF => let casted_var= a as type.Example: let b= a as u8;
//4.Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior.
//5.The behavior of all casts between integral types is well defined in rust-lang.

//NOTE: When casting from negative integer to unsigned integer is modular arithmetics =>
// let a_unsigned= a uint; => a_unsigned_value= a_value mod(max of uint)
fn main() {
    //In this block I perform casting from i8 to u8
    let a: i8 = -101;

    //If a < 0 ,its modular arithmetic with m=256 => value is 155 because:
    // -101 mod(256)=>155
    let a_unsigned = a as u8;

    //Below I perform casting from :
    //i8 => i32
    let b: i32 = a as i32;

    //u8 => u32
    let b_unsigned = a_unsigned as u32;

    //i8 => u32
    let c_unsigned: u32 = a as u32;

    //u32 to i32
    let c = c_unsigned as i32;
    println!("a: {}, a_unsigned: {}", a, a_unsigned);
    println!("b: {}, b_unsigned: {}", b, b_unsigned);
    println!("c_unsigned: {}, c: {}", c_unsigned, c);
}
