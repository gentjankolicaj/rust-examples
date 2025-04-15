//1.In rust-lang we have reference concept, which means that we can refer to values of different types.
//2.In rust-lang reference notation is '&T, &mut T'
//3.In rust-lang we dereference with '*'

fn main() {
    //declare variable and variable binding on value f32
    let a = 0.0f32;

    //declare variable and variable binding on primitive reference
    let a_ref = &a;

    println!("a = {},a_ref={}", a, a_ref);

    //declare mutable variable and variable binding on value f32
    let mut b = 10.0f32;

    //declare mutable variable and variable binding on primitive reference
    let b_ref = &mut b;
    println!("b_ref={},*b_ref={}", b_ref, *b_ref);
}