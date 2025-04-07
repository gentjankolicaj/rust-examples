//1.In Rust numeric literals can be type annotated by adding type keyword as suffix of literal.
//Ex. let a = 10u8; where u8 is primitive type unsigned 8 bit
//2. In Rust unsuffixed numeric literal's type depends on how they are used.
//3. In Rust if for numeric literal no constraints exist compiler will use i32 for integers & f64 for floats.
//4. This means that default type in Rust for integer is i32 and for float is f64,

fn main() {
    //unsuffixed literal type
    let a = 10;
    let b = 2.0;

    println!("a = {}", a);
    println!("b = {}", b);

    //suffixed literal's type
    let c = 100i128; //type i128
    let d = 1000.0f32; //type f32
    println!("suffixed type c = {}", c);
    println!("suffixed type d = {}", d);
}