//1.In rust-lang we can pass 'closures' as function parameters.
//2.In rust-lang when we use closure as params, we must also specify how closures capture values.
//3.In rust-lang we specify how closure params capture values at function head.
//3.In rust-lang we use the below keywords to specify of closures:
/*
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
*/

//Functions that take 'closure' F as parameter
//<F> denote that F is a "Generic type parameter"
//'where F:FnOnce' specifies that the closure uses the captured value by value (T)
fn apply_void<F>(void_closure: F)
where
    F: FnOnce(),
{
    //closure takes not params and returns nothing
    void_closure();
    println!("void_closure called()")
}

//Functions that take 'closure' F as parameter
//<F> denote that F is a "Generic type parameter"
//'where F:Fn' specifies that the closure uses the captured value by reference (&T)
fn apply_int<F>(int_closure: F, x: i32)
where
    F: Fn(i32) -> i32,
{
    let output = int_closure(x);
    println!("int_closure output={}", output);
}

fn main() {
    let one = 1;

    //declare closure with no param & return
    let void_closure = || {};

    //declare closure with param & return
    let int_closure = |x: i32| x + 1;

    //call functions that receive closures as params

    apply_void(void_closure);
    apply_int(int_closure, 10);
}
