//1.In rust-lang closures are functions that can capture the enclosing environment.
//2.Closure BNF: "|" <function_params>  "|" -> <return_type> "{" <function_body> "}"
//3.A regular function can't refer to variables in the enclosing environment

fn main() {
    let one = 1;

    //declare closures
    //annotated and inferred closures
    let empty_closure = || one;
    let annotated_closure = |param: i32| -> i32 { param + one };
    let inferred_closure = |param| -> i32 { param + one + 1 };

    //call closures
    println!("empty_closure: {}", empty_closure());
    println!("annotated_closure={}", annotated_closure(1));
    println!("inferred_closure={}", inferred_closure(1));
}
