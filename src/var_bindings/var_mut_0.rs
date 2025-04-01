//1.In Rust variable bindings are immutable by default.Ex. let A=value; => A is immutable
//2.Variable binding mutability can be changed with keyword 'mut'.
//3.Keyword 'mut' makes variable mutable.BNF of 'mut' :
/**
  <mut-expression> ::= "let" "mut" <assigment-expression>
  <assigment-expression> ::= <identifier> "=" <literal-primitive> | <identifier> ":" <primitive-type-keyword> "=" <literal-primitive>
  <identifier> ::= [a-zA-Z0-0]
  <literal-primitive> ::= (to be researched)
  <primitive-type-keyword> ::= "i8" | "i16" | "i32" | "i64" ...
*/

fn main(){
    //default variable binding is immutable in rust
    let a=12;
    let b =3.14;

    //Define mutable variable
    let mut c=a;
    let mut d=b;
    c=c+10;
    d=d+20.0; //need to be float literal else compile error
    println!("a:{},b:{},c:{:?},d:{:?}",a,b,c,d);
}