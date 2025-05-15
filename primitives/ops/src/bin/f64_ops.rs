fn main() {
    let a: f64 = 10.0;
    let b: f64 = 8.0;
    let c: f64 = -1.0;
    let mut d: f64 = 0.0;

    //print in different formats
    println!("a in different bases=> decimal:{} ", a,);
    println!("b in different bases=> decimal:{} ", b,);

    //arithmetic operators
    println!("arithmetic operators : ");
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    //unary operators
    print!("unary operators : ");
    println!("c in different bases=> decimal:{} ", c,);
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
    //println!("AND-OP=> a=[{}] , b=[{}] , a & b =[{}] ",a, b,a & b); error[E0369]: no implementation for `f64 & f64'
    // println!("OR-OP=> a=[{}] , b=[{}] , a | b =[{}] ",a, b,a | b); error[E0369]: no implementation for `f64 | f64'
    // println!("XOR=> a=[{}] , b=[{}] , a ^ b =[{}] ",a,  b,a ^ b); error[E0369]: no implementation for `f64 ^ f64'
    // println!("LEFT-SHIFT=> 1<<7 =[{}] ",1f64<<7); error[E0369]: no implementation for `f64 <<
    // println!("RIGHT-SHIFT=> 127>>7 =[{}] ",127f64>>6); error[E0369]: no implementation for `f64 >>
}
