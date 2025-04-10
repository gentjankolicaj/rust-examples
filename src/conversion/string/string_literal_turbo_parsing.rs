fn main() {
    //parsing string literal to i32
    let a: i32 = "5".parse::<i32>().unwrap();

    //turbo parsing string literal to i32
    let b: i32 = "7".parse::<i32>().unwrap();

    let c = a + b;
    println!("a+b={}", c);
}
