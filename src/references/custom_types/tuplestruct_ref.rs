//1.In rust-lang we have reference concept, which means that we can refer to values of different types.
//2.In rust-lang reference notation is '&T, &mut T'
//3.In rust-lang we dereference with '*'

#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    //Point: declare variable,instantiate and assign value
    let a = Point(0, 0);

    //Point reference: declare variable,assign reference value
    let a_ref = &a;

    println!("a = {:?},a_ref={:?}", a, a_ref);

    //Point: declare mutable variable,instantiate and assign value
    let mut b = Point(2, 2);

    //Point mutable reference: declare variable,assign reference value
    let b_ref = &mut b;
    println!("b_ref={:?},*b_ref={:?}", b_ref, *b_ref);
}