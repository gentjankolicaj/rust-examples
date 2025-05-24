//1.In rust-lang <T> declaration implies that T is generic type.
//2.In rust-lang generics can by bounded traits with declaration <T: TraitType>
//3.In rust-lang bounding generics with traits helps to stipulate behaviour of type.


// define 2 traits to be used as generics parameter bounds
trait T1{

}
trait T2{

}

fn  print_single_bound<G: T1>(param: &G){
    println!("Called print_single_bound(),param is generic reference bounded by [T1]");
}

fn print_multiple_bound<G: T1+T2>(param: &G){
    println!("Called print_multiple_bound(),param is generic reference bounded by [T1+T2]");
}

//Declare primitives that implement 2 traits in order to pass as params

impl T1 for i32 {}
impl T2 for i32 {}

impl T1 for f64 {}
impl T2 for f64{}

fn main() {
    let a=10;
    let b=3.14;

    //call method print_single_bound() which accepts parameters generic type references bound by T1 trait.
    print_single_bound(&a);
    print_single_bound(&b);

    //call method print_single_bound() which accepts parameters generic type references bound by T1+T2 trait.
    print_multiple_bound(&a);
    print_multiple_bound(&b);
}