//1.In rust-lang we have reference concept, which means that we can refer to values of different types.
//2.In rust-lang reference notation is '&T, &mut T'
//3.In rust-lang we dereference with '*'

fn main() {
    //declare variable and variable binding on value i32
    let a = 0i32;

    //declare variable and variable binding on primitive reference
    let a_ref = &a;

    println!("a = {},a_ref={}", a, a_ref);

    //declare mutable variable and variable binding on value i32
    let mut b = 1i32;

    //declare mutable variable and variable binding on primitive reference
    let b_ref = &mut b;
    println!("b_ref={},*b_ref={}", b_ref, *b_ref);

    // //failing code
    //
    // b=b+1;
    // println!("after b=b+1 , b = {},b_ref={}",b,b_ref);
    //
    // //b_ref=b_ref+1; //compile error : error[E0369]: cannot add `{integer}` to `&mut i32`
    // //I dereference then add and assign
    // *b_ref=*b_ref+1;
    // println!("after b_ref=b_ref+1 , b = {},b_ref={}",b,b_ref);
    //

}