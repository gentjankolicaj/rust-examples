//1.In rust-lang <T> declaration implies that T is generic type.
//2.In rust-lang generics can by bounded traits with declaration <T: TraitType>
//3.In rust-lang bounding generics with traits helps to stipulate behaviour of type.
//4.In rust-lang '::' is a path separator to access: ( modules , functions , types ,  associated functions/constants ,enums )


//rust-lang exiting traits
use std::fmt::{
    Debug,
    Display
};

fn  print_display<G: Display>(param: &G){
   println!("print_display(): {}",param);
}

fn print_debug<T: Debug>(param: &T){
    println!("print_debug(): {:?}", param);
}

fn print_both_traits<T: Debug + Display>(param: &T){
    println!("print_both_traits(), display print {}", param);
    println!("print_both_traits(), debug print {:?}", param);
}


fn main() {
   let a=10;
    let b=3.14;

    //pas i32 reference as parameter since it implements traits Debug,Display
    print_display(&a);
    print_debug(&a);
    print_both_traits(&a);

    //pass f64 reference as parameter since it implements traits Debug,Display
    print_display(&b);
    print_debug(&b);
    print_both_traits(&b);

}