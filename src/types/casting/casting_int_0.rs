//1.rust-lang provides several mechanism to change or define type of a primitive and user defined types.
//2.Options :
//2.1 Casting between primitive types
//2.2 Specifying the desired type of literals
//2.3 Using type inference
//2.4 Aliasing types

//NOTE: Casting is explicit type conversion of primitives in rust.
//rust-lang does not provide implicit type casting (coercion) between primitives.
//CASTING BNF => let casted_var= a as type.Example: let b= a as u8;

fn main() {
    //In this block I perform casting from i8 to u8
    {
        let a_positive = 1i8;
        let a_unsigned = a_positive as u8;
        println!("a_positive: {}, a_unsigned: {}", a_positive, a_unsigned);
    }

    //In this block I perform casting from i16 to u16
    {
        let b_positive = 2i16;
        let b_unsigned: u16 = b_positive as u16;
        println!("b_positive: {}, b_unsigned: {}", b_positive, b_unsigned);
    }

    //In this block i perform casting of i32 to u32
    {
        let c_positive = 10;

        //casting to unsigned int
        let c_unsigned = c_positive as u32;
        println!("c_positive:{},c_unsigned:{}", c_positive, c_unsigned);
    }
}
