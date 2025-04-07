//NOTE: In programming 'Type-Inference' refers to process of automatically
// assigning a type to each expression in a program based on context and structure of program.
//1.In Rust 'type-inference' engine is smart.
//2.It does more than looking at the type of the value expression during an initialization.
//3.It also looks at how the variable is used afterwards to infer its type.

//TUPLE-STRUCT
//Derive debug trait, for print
#[derive(Debug)]
struct Coord(f64, f64);

fn main() {
    //In below example compiler does 'type inference' of vec based on parameter type on function.
    // Instantiate a tuple-struct of type Coord
    let tirane_coord = Coord(41.327953, 19.81902);

    //Declare another variable 'v'
    //v is a empty growable array and it will hold values
    let mut v = Vec::new();
    //at this point compiler doesn't know that type of 'v',it just knows that it's a vector of something

    //By calling function push and passing variable binding tirane_coord
    //Compiler infers that type of 'v' is Coord.
    v.push(tirane_coord);

    println!("{:?}", v);
}