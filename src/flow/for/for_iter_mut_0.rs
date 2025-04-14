//1.In rust-lang the 'for in' construct is able to interact with an Iterator in several ways.
//2.In rust-lang by default the for loop will apply the 'into_iter' function to the collection.
//3.In rust-lang 'iter_mut' func mutably borrows each element of the collection, allowing for the collection to be modified in place.

fn main() {
    let mut int_vals = vec![1, 2, 3, 4, 5];

    for int in int_vals.iter_mut() {
        if *int == 3 {
            *int = 007;
        } else {
            *int = 008;
        }
        println!("{}", int);
    }

    println!("int_vals: {:?}", int_vals);
}
