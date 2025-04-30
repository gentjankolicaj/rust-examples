//1.In rust-lang we can pass 'closures' as function parameters.
//2.In rust-lang when we use closure as params, we must also specify how closures capture values.
//3.In rust-lang we specify how closure params capture values at function head.
//3.In rust-lang we use the below keywords to specify of closures:
/*
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
*/
//4.In rust-lang 'move' keyword when be used, signals that all captures occur by value

fn main() {
    let a_array = [1, 2, 3, 4];

    let two_idx = a_array.iter().position(|&x| x == 2);
    println!("two_idx={:?}", two_idx);

    let three_idx = a_array.into_iter().position(|x| *x == 3);
    println!("three_idx={:?}", three_idx);
}
