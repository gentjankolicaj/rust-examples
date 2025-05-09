//1.In rust-lang by default, the items in a module have private visibility, but this can be overridden with the pub modifier
//2.In rust-lang 'pub' is a access modifier of items in module { functions, structs, enums, traits, modules )
//3.In rust-lang only the public items of a module can be accessed from outside the module scope.
//4.In rust-lang module EBNF:
// <rust-module> ::= <access-modifier> <module-name> "{" <module-items>+ "}"
// <module-items> ::= { <module-item> }
// <module-item> ::=  <function> | <struct> | <enum> | <trait> | <module>

mod a {

    fn info() {
        println!("function a() in module A");
    }

    //tuple-like struct
    struct Pair(i32, i32);

    enum Color {
        Red = 0,
        Green = 1,
        Blue = 2,
    }

    trait Greetable {
        fn greet(&self);
    }

    mod b {
        fn info() {
            println!("function b() in module B, inside module a");
        }
    }
}

fn main() {
    //call items of module a
    // a::info(); //compile error because of private visibility
    // let a_pair= a::Pair(0,1); //compile error because of private visibility
    // let a_color= a::Color::Red; //compile error because of private visibility

    // //implement trait for struct Pair
    // impl Greetable for a::Pair {
    //     fn greet(&self) {
    //         println!("Implemented Greetable trait for struct Pair!");
    //     }
    // }
    // a_pair.greet();  //compile error because of private visibility
    //
    // //call items of module b
    // a::b::info(); //compile error because of private visibility

    println!("Module items need public visibility to be accessed from outside the module scope.");
}
