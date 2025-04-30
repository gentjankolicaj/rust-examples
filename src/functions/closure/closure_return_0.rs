//1.In rust-lang we can pass 'closures' as function parameters.
//2.In rust-lang when we use closure as params, we must also specify how closures capture values.
//3.In rust-lang we specify how closure params capture values at function head.
//3.In rust-lang we use the below keywords to specify of closures:
/*
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
*/
//4.In rust-lang 'move' keyword must be used, which signals that all captures occur by value

fn closure_output_function() -> impl FnOnce() {
    println!("called: closure_output_function()");
    println!("returning a closure that captures by value.");

    //declare variable, by default variables are immutable.
    let int = 10;

    //create closure.
    let var = move || {
        println!("closure called with value:{}", int);
    };

    //return closure
    var
}

fn main() {
    let closure_output = closure_output_function();
    closure_output();
}
