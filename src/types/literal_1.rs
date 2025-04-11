//1.In rust-lang numeric literals can be type annotated by adding type keyword as suffix of literal.
//Ex. let a = 10u8; where u8 is primitive type unsigned 8 bit
//2. In rust-lang unsuffixed numeric literal's type depends on how they are used.
//3. In rust-lang if for numeric literal no constraints exist compiler will use i32 for integers & f64 for floats.
//4. This means that default type in rust-lang for integer is i32 and for float is f64,

//NOTE: std::mem::size_of_val is a function, but called with its full path.
//1.code can be split in logical units called modules
//2.size_of_value is defined in (logic unit) module 'mem' and module 'mem' is defined in crate 'std'

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

    // `size_of_val` returns the size of a variable in bytes
    println!("size_of_value a= {}", std::mem::size_of_val(&a));
    println!("size_of_value b = {}", std::mem::size_of_val(&b));
    println!("size_of_value c = {}", std::mem::size_of_val(&c));
    println!("size_of_value d = {}", std::mem::size_of_val(&d));
}
