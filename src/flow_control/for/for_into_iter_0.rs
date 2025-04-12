//1.In rust-lang the 'for in' construct is able to interact with an Iterator in several ways.
//2.In rust-lang by default the for loop will apply the 'into_iter' function to the collection.
//3.In rust-lang 'into_iter' function consumes the collection so that on each iteration the exact data is provided.

fn main() {
    let int_vals = vec![1, 2, 3, 4, 5];

    //cloning because of:
    //  -------- move occurs because `int_vals` has type `Vec<i32>`, which does not implement the `Copy` trait
    // println!("int_vals: {:?}", int_vals); |                                ^^^^^^^^ value borrowed here after move
    for int in int_vals.clone().into_iter() {
        println!("{}", int);
    }

    println!("int_vals: {:?}", int_vals);
}