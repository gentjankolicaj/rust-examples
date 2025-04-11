//1.In rust-lang we use 'type' keyword to give a new name to existing types.
//2.New names of existing type (aliases) must be UpperCamelCase or compiler will throw compile warning.
//3.Exception to naming rules 2 are aliases (new names) for primitive types: Ex: type I8 as i8;

//4.Aliasing statement BNF =>
// <aliasing-statement> ::= <type-keyword> " " <new-name> "=" <existing-type>
// <type-keyword> ::= "type"
// <new-name> ::= [A-Za-z0-9] | [A-Z0-9]
// <existing-type> ::= <primitive-type> | <custom-type>
// <primitive-type> ::= "i8" | "u8" | "i16" | "u16 ....
// <custom-type>  :: = <struct-type> | <enum-type>

//NOTE: WE USE ALIASES TO REDUCE BOILERPLATE CODE.
//NOTE: TYPE ALIASES CAN'T BE USED AS CONSTRUCTOR,ELSE COMPILE ERROR

#[derive(Debug, Clone, Copy)]
enum Switch {
    ON,
    OFF,
}

//Aliasing
type SWITCH = Switch;

fn main() {
    //Get enum with alias
    let a = SWITCH::ON;
    println!("a={:?}", a);
}
