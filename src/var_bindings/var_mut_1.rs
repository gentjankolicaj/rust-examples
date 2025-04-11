//1.In rust-lang variable bindings are immutable by default.Ex. let A=value; => A is immutable
//2.Variable binding mutability can be changed with keyword 'mut'.
//3.Keyword 'mut' makes variable mutable.BNF of 'mut' :
/**
<mut-expression> ::= "let" "mut" <assigment-expression>
<assigment-expression> ::= <identifier> "=" <literal-primitive> | <identifier> ":" <primitive-type-keyword> "=" <literal-primitive>
<identifier> ::= [a-zA-Z0-0]
<literal-primitive> ::= (to be researched)
<primitive-type-keyword> ::= "i8" | "i16" | "i32" | "i64" ...
*/

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1; compile error.
}
