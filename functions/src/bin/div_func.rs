//As opposed to all the other types, this one cannot be instantiated,
//The set of all possible values this type can have is empty.
fn diverging_func() -> ! {
    println!("In rust-lang type '!' is used to denote a diverging function");
    println!("A diverging function is a function that never returns");

    panic!("diverging_func()");
}

fn main() {
    //call a defined diverging function.
    diverging_func();
}
