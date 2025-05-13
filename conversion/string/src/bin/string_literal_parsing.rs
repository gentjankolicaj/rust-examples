fn main() {
    //parsing string literal to i32
    let a: i32 = "5".parse().unwrap();

    //turbo parsing string literal to i32
    let b: i32 = "6".parse().unwrap();

    let c = a + b;
    println!("a+b={}", c);
}
