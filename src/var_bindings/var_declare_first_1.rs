//1.In Rust is possible to declare variable bindings (identifiers) first and initialize them later
//2.In Rust all variable bindings must be initialized before they are used.
//3.Compiler forbids use of uninitialized var bindings as it would lead to undefined behaviour.
//4.It is not common to declare a variable binding and initialize it later in the function or block
//5.It is more difficult for a reader to find the initialization when initialization is separated from declaration.
//6.It is common to declare and initialize a variable binding near where the variable will be used.

fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    //  println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
