//1.In rust-lang, we have 'block's.
//2.In rust-lang a 'block' is a collection of statements enclosed by {}
//3.In rust-lang variable bindings have a scope and live in a 'block'.
//4.rust-lang 'block' BNF :
/**
<rust-blocks> ::= <rust-block>...<rust-block>
<rust-blocks> ::= "{" <collection-statements> "}"
<collection-statements> ::= <rust-statement> ... <rust-statement>
*/
fn print() {
    let print_block = true;
    println!("variable-binding print_block={}, exists in print() block therefore you can't operate on it from outside print().", print_block);
}
fn main() {
    //variable binding is in main block => main_var visibility in main
    let main_block = 1;
    {
        let inside_block = 2;
        println!("inside_block={}", inside_block);
    }

    println!("main_block={}", main_block);
    // println!("inside_block={}",inside_block); Compile error : error[E0425]: cannot find value `inside_block` in this scope
    //println!("print_block={}",print_block); Compile error : because print_block is on different block => different scope.
}
