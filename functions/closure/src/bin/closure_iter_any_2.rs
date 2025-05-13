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
    let a_vec = vec![1, 2, 3, 4];
    //iter() yields of &i32
    println!(
        "a_vec has 2 by reference : {}",
        a_vec.iter().any(|&x| x == 2)
    );

    //into_iter() yield type i32
    println!(
        "a_vec has 2 by value : {}",
        a_vec.into_iter().any(|x| x == 2)
    );

    //iter_mut() for yield type &mut i32, thats why we need to deference with '*'
    //can'b be borrowed as mutable, a_vec must be mutable
    // println!("a_vec has 2 by reference : {}",a_vec.iter_mut().any(|x| *x == 2));
}
