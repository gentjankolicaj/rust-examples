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

//Declare structs that implement 2 traits in order to passed as params
struct Unit;
struct Pair(i32, i32);

impl T1 for Unit {}
impl T2 for Unit {}

impl T1 for Pair {}
impl T2 for Pair{}

fn main() {
    let u0 = Unit;
    let p0 = Pair(0, 0);

    //call method print_single_bound() which accepts parameters generic type references bound by T1 trait.
    print_single_bound(&u0);
    print_single_bound(&p0);

    //call method print_single_bound() which accepts parameters generic type references bound by T1+T2 trait.
    print_multiple_bound(&u0);
    print_multiple_bound(&p0);
}