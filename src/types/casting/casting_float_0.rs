//1.Casting is explicit type conversion of primitives in Rust.
//2.Rust does not provide implicit type casting (coercion) between primitives.
//3.CASTING BNF => let casted_var= a as type.Example: let b= a as u8;
//4.Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior.
//5.The behavior of all casts between integral types is well defined in Rust.

fn main() {
    {
        //In this block I perform casting from f32 to f64
        let a_f32: f32 = 3.14;
        let a_f64: f64 = a_f32 as f64;

        println!("a_f32:{} to a_f64:{}", a_f32,a_f64);
    }

    {
        //In this block I perform casting from f64 to f32
        let a_f64: f64 = 3.1423456723454567;
        let a_f32: f32 = a_f64 as f32;

        println!("a_f64:{} to a_32:{}",a_f64,a_f32);
    }

}
