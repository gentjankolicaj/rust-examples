fn main() {
    let int_literal = 10;
    println!("int_literal: {}", int_literal);

    let binary_literal = 0b10;
    println!("binary_literal: {}", binary_literal);

    let octal_literal = 0o10;
    println!("octal_literal: {}", octal_literal);

    let decimal_literal = 10;
    println!("decimal_literal: {}", decimal_literal);

    let hex_literal = 0x10;
    println!("hex_literal: {}", hex_literal);

    //int literal with suffixed type
    let literal_int_suffixed = 10i32;
    println!("literal_int_suffixed: {}", literal_int_suffixed);

    //long literal with suffixed type
    let literal_long_suffixed = 10i64;
    println!("literal_long_suffixed: {}", literal_long_suffixed);

    let literal_underscore = 1_000_000;
    println!("literal_underscore: {}", literal_underscore);
}
