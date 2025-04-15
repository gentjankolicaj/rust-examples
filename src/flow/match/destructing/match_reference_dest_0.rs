//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
//7.'match' can be used to destruct enums
//8.'match' can be used to destruct c-structs
//9.'match' can be used to destruct reference with (&, ref, and ref mut)

fn main() {

    //assign a reference to a primitive type i32
   let reference=&10; //of type i32 reference

    match reference {
        // If `reference` is pattern matched against `&val`, it results in a comparison like: `&i32` and `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }


}