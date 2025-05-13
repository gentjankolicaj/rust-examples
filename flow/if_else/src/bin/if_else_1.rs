//1.In rust-lang we control flow with if-else conditionals.
//2.In rust-lang if-else conditionals are expressions, and, all branches must return the same type from blocks.

fn main() {
    //Below if-else expressions, return_val=1 where type is i32
    let int = 10;

    let return_val = if int < 0 {
        println!("int < 0");

        //return value -1, if ';' is used then return value is ()
        -1
    } else if int == 0 {
        println!("int = 0");

        //return value 0, if ';' is used then return value is ()
        0
    } else {
        println!("int > 0");

        //return value 1, if ';' is used then return value is ()
        //if place ';' => Compile error ^ expected `()`, found integer
        1
    }; //All bindings need a semi-column ';'

    println!("return_val ={}", return_val);
}
