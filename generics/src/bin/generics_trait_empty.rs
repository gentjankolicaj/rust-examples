//1.In rust-lang <T> declaration implies that T is generic type.
//2.In rust-lang generics can by bounded traits with declaration <T: TraitType>
//3.In rust-lang bounding generics with traits helps to stipulate behaviour of type.


//set file level scope attribute
#![warn(dead_code)]

//define 3 types of structs & implement a trait with generics
#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B(A);

#[derive(Debug)]
struct C {
    a: A,
    b: B,
}

//Define 2 traits with no functions
trait TA{

}

trait TB{

}

//implement empty trait TA
impl TA for A{
}
impl TA for B{
}
impl TA for C{
}

//implement empty trait TB
impl TB for A{
}
impl TB for B{

}
impl TB for C{
}

//accepts parameter with implements trait TA
fn apply_ta<T: TA>(param: &T){
    println!("applyTA() called");
}

//accepts parameter with implements trait TA
fn apply_tb<T: TB>(param: &T){
    println!("applyTB() called");
}

fn main() {
    let a = A;
    let b = B(A);
    let c = C { a: A, b: B(A) };

    //call function that accepts generic parameters bounded by TA trait
    apply_ta(&a);
    apply_ta(&b);
    apply_ta(&c);

    //call function that accepts generic parameters bounded by TB trait
    apply_tb(&a);
    apply_tb(&b);
    apply_tb(&c);


}
