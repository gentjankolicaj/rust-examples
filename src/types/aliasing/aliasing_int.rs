//1.In rust-lang we use 'type' keyword to give a new name to existing types.
//2.New names of existing type (aliases) must be UpperCamelCase or compiler will throw compile warning.
//3.Exception to naming rules 2 are aliases (new names) for primitive types: Ex: type I8 as i8;

//4.Aliasing statement BNF =>
// <aliasing-statement> ::= <type-keyword> " " <new-name> "=" <existing-type>
// <type-keyword> ::= "type"
// <new-name> ::= [a-Za-z0-9] | [a-Z0-9]
// <existing-type> ::= <primitive-type> | <custom-type>
// <primitive-type> ::= "i8" | "u8" | "i16" | "u16 ....
// <custom-type>  :: = <struct-type> | <enum-type>

//NOTE: WE USE ALIASES TO REDUCE BOILERPLATE CODE.

type Byte = i8;
type Short = i16;
type Int = i32;
type Long = i64;

fn main() {
    let a: Byte = 10;
    let b: Short = 20;
    let c: Int = 30;
    let d: Long = 40;

    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}
