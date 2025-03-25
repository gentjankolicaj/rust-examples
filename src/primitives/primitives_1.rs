fn main() {
    //integer addition
    println!("1 + 2 = {}", 1i32 + 2);

    //unsigned integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    //integer subtraction
    print!("1 - 2 = {}", 1i32 - 2);

    //unsigned integer subtraction
    // print!("1 - 2 = {}",1u8-2); causes: attempt to compute `1_u32 - 2_u32`, which would overflow because u8 value[0,255]

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("short-circuit: true AND false is {}", true && false);
    println!("short-circuit: true OR false is {}", true || false);
    println!("short-circuit: NOT true is {}", !true);

    // Bitwise ops
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}