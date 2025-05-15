//NOTE: In programming 'Type-Inference' refers to process of automatically
// assigning a type to each expression in a program based on context and structure of program.
//1.In rust-lang 'type-inference' engine is smart.
//2.It does more than looking at the type of the value expression during an initialization.
//3.It also looks at how the variable is used afterwards to infer its type.

//UNIT-STRUCT
//Derive debug trait, for print
#[derive(Debug)]
struct Unit;

fn main() {
    //In below example compiler does 'type inference' of vec based on parameter type on function.
    // Instantiate a unit struct
    let unit_var = Unit;

    //Declare another variable 'v'
    //v is a empty growable array and it will hold values
    let mut v = Vec::new();
    //at this point compiler doesn't know that type of 'v',it just knows that it's a vector of something

    //By calling function push and passing variable binding unit_var
    //Compiler infers that type of 'v' is Unit.
    v.push(unit_var);

    println!("{:?}", v);
}
