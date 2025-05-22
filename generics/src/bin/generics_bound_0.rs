//1.In rust-lang <T> declaration implies that T is generic type.
//2.In rust-lang generics can by bounded traits with declaration <T: TraitType>
//3.In rust-lang bounding generics with traits helps to stipulate behaviour of type.


use std::fmt::Debug;


//below is a function with generic parameter & this generic is bounded to Debug trait
fn print_type<T: Debug>(param: T) {
    println!(
        "print_type function with generic parameter bounded: {:?}",
        param
    );
}

fn main() {
    let int = 1;
    let float = 3.14;
    print_type(int);
    print_type(float);
}
