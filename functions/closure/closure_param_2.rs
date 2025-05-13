//1.In rust-lang we can pass 'closures' as function parameters.
//2.In rust-lang when we use closure as params, we must also specify how closures capture values.
//3.In rust-lang we specify how closure params capture values at function head.
//3.In rust-lang we use the below keywords to specify of closures:
/*
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
*/

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}
