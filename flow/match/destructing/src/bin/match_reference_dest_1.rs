//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
//7.'match' can be used to destruct enums
//8.'match' can be used to destruct c-structs
//9.'match' can be used to destruct reference with (&, ref, and ref mut)

//Define c-struct
#[derive(Debug)]
struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    //instantiate Foo and assign a reference value
    let reference = &Foo { x: (1, 2), y: 3 };

    //destructing of reference with match
    match reference {
        // If `reference` is pattern matched against `&val`, it results in a comparison like: `&Foo` and `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        val => println!("Got a value via destructuring: {:?}", val),
    }

    //destructing of reference with match
    //deference with *
    match reference {
        // If `reference` is pattern matched against `&val`, it results in a comparison like: `&Foo` and `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        val => println!("Got a value via destructuring and dereference : {:?}", *val),
    }
}
