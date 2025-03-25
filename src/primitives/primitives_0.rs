fn main() {

    //==============================================================================================
    //Scalar types
    //==============================================================================================
    let a: i8 = 8;
    let b: i16 = 16;
    let c: i32 = 32;
    let d: i64 = 64;
    let e: i128 = 128;
    let f: f32 = 3.2;
    let g: f64 = 6.4;
    let h: char = 'h';
    let i: bool = true;

    //macro to print
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);
    println!("i: {}", i);

    //suffix annotation
    let an_integer = 5i32;
    println!("an integer: {}", an_integer);

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    println!("default_float: {}", default_float);
    println!("default_integer: {}", default_integer);

    // Type i64 is inferred from another line.
    // Variable needs to be mutable with mut keyword
    let mut inferred_type = 11;
    inferred_type = 4294967296i64;
    println!("inferred_type: {}", inferred_type);

    //mutable local variable
    let mut mutable = 0;
    mutable = 1;
    println!("mutable: {}", mutable);


    //===============================================================================================
    //COMPOUND TYPES
    //==============================================================================================

    //Rust arrays
    //declaration array ::=let identifier:[type;length]
    //declaration and initialization array::=let identifier:[type;length]=[v1,v2...]
    let _null_array: [i32; 0];

    let _empty_array: [i32; 0] = [];
    println!("EMPTY array: {:?}", _empty_array);

    let int_array: [i32; 4] = [5, 6, 7, 1];
    println!("int_array: {:?}", int_array);

    //Declare long array with length 11 and populate with -1 on all indexes
    let long_array: [i64; 11] = [-1; 11];
    println!("long_array: {:?}", long_array);

    //Tuple is a collection of values of different types
    //rust tuple ::= let identifier=(t1,t2,t3);

    let simple_tuple = (10u8, 0u64, true, 3.14f32);
    println!("simple_tuple: {:?}", simple_tuple);
}