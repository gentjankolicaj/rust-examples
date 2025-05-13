//1.In rust-lang we control flow with if-else conditionals.
//2.In rust-lang if-else conditionals are expressions, and, all branches must return the same type from blocks.

fn main() {
    let int = 10;
    if int < 0 {
        println!("int < 0");
    } else if int == 0 {
        println!("int = 0");
    } else if int > 0 {
        println!("int > 0");
    }
}
