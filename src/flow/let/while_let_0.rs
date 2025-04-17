//1.In rust-lang 'while let' construction is similar to if let
//2.This means that 'while-let' can be used for destructing

fn main() {
    let optional = Some(1);

    //Destruct variable optional at Some(x)=optional
    while let Some(x) = optional {
        if x == 1 {
            println!("The value of optional_value is: {}", x);
            println!("'break' while loop.");
            break;
        } else {
            println!("The value of optional_value is: {}", x);
            println!("'continue' while loop.");
            continue;
        }
    }
}