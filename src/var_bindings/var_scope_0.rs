//1.In Rust, we have 'block's.
//2.In Rust a 'block' is a collection of statements enclosed by {}
//3.In Rust variable bindings have a scope and live in a 'block'.
//4.Rust 'block' BNF :
/**
  <rust-blocks> ::= <rust-block>...<rust-block>
  <rust-blocks> ::= "{" <collection-statements> "}"
  <collection-statements> ::= <rust-statement> ... <rust-statement>
*/

fn main(){
    //variable binding is in main block => main_var visibility in main
    let main_block=1;
    {
      let inside_block=2;
        println!("inside_block={}",inside_block);
    }

    println!("main_block={}",main_block);
    // println!("inside_block={}",inside_block); Compile error : error[E0425]: cannot find value `inside_block` in this scope
}