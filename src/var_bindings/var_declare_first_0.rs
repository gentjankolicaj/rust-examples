//1.In Rust is possible to declare variable bindings (identifiers) first and initialize them later
//2.In Rust all variable bindings must be initialized before they are used.
//3.Compiler forbids use of uninitialized var bindings as it would lead to undefined behaviour.
//4.It is not common to declare a variable binding and initialize it later in the function or block
//5.It is more difficult for a reader to find the initialization when initialization is separated from declaration.
//6.It is common to declare and initialize a variable binding near where the variable will be used.

fn sample() {
    //var bindings declared first
    let var_binding_a;
    let var_binding_b;

    //initialized in function
    var_binding_a = 32i8;

    {
        //initialized in a nested block
        var_binding_b = 0;
        let x = var_binding_b + 101;
        println!("x={}", x);
    }

    println!("var_binding_a={}", var_binding_a);
    println!("var_binding_b={}", var_binding_b);

    let var_binding_c: i64;

    //compile error because it is forbidden to use uninitialized variable bindings
    //print!("var_binding_c={}",var_binding_c); =>consider giving `var_binding_c` an explicit type
}

fn main() {
    sample();
}