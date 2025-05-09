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
//NOTE: TYPE ALIASES CAN'T BE USED AS CONSTRUCTOR,ELSE COMPILE ERROR

//unit-struct
#[derive(Debug, Clone, Copy)]
struct Unit;

//tuple-struct
#[derive(Debug, Clone, Copy)]
struct Pair(i32, i32);

//c-struct
#[derive(Debug, Clone, Copy)]
struct Point {
    lat: f64,
    lon: f64,
}

//Aliasing
type UNIT = Unit;

type PAIR = Pair;
type POINT = Point;

fn main() {
    //Instantiate different types of structs with aliasing
    let a: UNIT = Unit;
    let b: PAIR = Pair(1, 2);
    let c: POINT = Point { lat: 0.0, lon: 1.0 };
    println!("a={:?} b={:?} c={:?}", a, b, c);
}
