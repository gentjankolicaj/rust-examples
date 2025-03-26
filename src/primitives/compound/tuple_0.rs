//1.A tuple is a collection of values of different types.
//2.Tuples are constructed using parenthesis.
//3.Each tuple itself is a value with type signature (T1,T2,T3...) where T1,T2 are types of its members.
//4.Functions can use tuples to return values as tuples hold any number of values.

//function definition with arg/return type of tuple
fn reverse_type(arg: (i32, bool)) -> (bool, i32) {
    let (int_arg, boolean_arg) = arg;
    return (boolean_arg, int_arg);
}

//Define struct
#[derive(Debug)]
#[warn(dead_code)]
struct Matrix(f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types.
    let mix_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    //values can be extracted from tuple using indexing.
    println!("mix_tuple index 5 = {}", mix_tuple.5);
    println!("mix_tuple index 0 = {}", mix_tuple.0);

    let simple_tuple = (1i32, true);
    println!("simple_tuple= {:?}", simple_tuple);
    println!("simple_tuple reversed = {:?}", reverse_type(simple_tuple));

    let struct_tuple = (
        Matrix(1.1, 2.2, 3.3),
        true,
        false,
        (true, Matrix(3.12, 1.1, 0.1)),
    );
    println!("struct_tuple= {:?}", struct_tuple);

    // Tuples can be destructured to create bindings.
    let tuple = (1, "nina", 5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
