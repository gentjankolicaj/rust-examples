fn main() {
    let a: i32 = 10;
    let b: i32 = 8;
    let c: i32 = -1;
    let mut d: i32 = 0;

    //print in different formats
    println!(
        "a in different bases=> decimal:{} , binary:{:b} , octal:{:o}",
        a, a, a
    );
    println!(
        "b in different bases=> decimal:{} , binary:{:b} , octal:{:o}",
        b, b, b
    );

    //arithmetic operators
    println!("arithmetic operators : ");
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    //unary operators
    print!("unary operators : ");
    println!(
        "c in different bases=> decimal:{} , binary:{:b} , octal:{:o}",
        c, c, c
    );
    d = -c;
    println!("- c = {}", d);

    //comparison operators
    println!("comparison operators : ");
    println!("a == b = {}", a == b);
    println!("a != b = {}", a != b);
    println!("a > b = {}", a > b);
    println!("a >= b = {}", a >= b);
    println!("a < b = {}", a < b);
    println!("a <= b = {}", a <= b);

    //boolean logical operators
    println!("short-circuit logical operators :");
    println!("(true , true) , true && true = {}", true && true);
    println!("(true , false) , true && true = {}", true && false);
    println!("(true , true) , true || true = {}", true || true);
    println!("(true , false) , true || true = {}", true || false);
    println!("(true) , !true = {}", !true);
    println!("(false) , !false = {}", !false);

    //bitwise operators
    println!("bitwise operators : ");
    println!(
        "AND-OP=> a=[{},{:b}] , b=[{},{:b}] , a & b =[{},{:b}] ",
        a,
        a,
        b,
        b,
        a & b,
        a & b
    );
    println!(
        "OR-OP=> a=[{},{:b}] , b=[{},{:b}] , a | b =[{},{:b}] ",
        a,
        a,
        b,
        b,
        a | b,
        a | b
    );
    println!(
        "XOR=> a=[{},{:b}] , b=[{},{:b}] , a ^ b =[{},{:b}] ",
        a,
        a,
        b,
        b,
        a ^ b,
        a ^ b
    );
    println!("LEFT-SHIFT=> 1<<7 =[{},{:b}] ", 1i32 << 7, 1i32 << 7);
    println!("RIGHT-SHIFT=> 127>>7 =[{},{:b}] ", 127i32 >> 6, 127i32 >> 6);
}
