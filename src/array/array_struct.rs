use std::mem::{align_of_val, size_of_val};

//unit struct
#[derive(Debug)]
struct A;

//tuple struct
#[derive(Debug)]
struct B(i32, i32);

//named field struct
#[derive(Debug)]
struct C {
    i: i32,
    j: i32,
}

fn main() {
    let a = A;
    let b = B(1, 2);
    let c = C { i: 1, j: 2 };

    //declare arrays that contain struct values
    let aa: [A; 2] = [A, A];
    let ab: [B; 2] = [B(0, 0), B(1, 1)];
    let ac: [C; 2] = [C { i: 0, j: 0 }, C { i: 1, j: 1 }];

    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);

    println!("aa={:?}", aa);
    println!("aa size_of_val={}", size_of_val(&aa));
    println!("aa align_of_val={}", align_of_val(&aa));

    println!("ab={:?}", ab);
    println!("ab size_of_val={}", size_of_val(&ab));
    println!("ab align_of_val={}", align_of_val(&ab));

    println!("ac={:?}", ac);
    println!("ac size_of_val={}", size_of_val(&ac));
    println!("ac align_of_val={}", align_of_val(&ac));
}
