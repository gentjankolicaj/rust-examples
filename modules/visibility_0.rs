//1.In rust-lang by default, the items in a module have private visibility, but this can be overridden with the pub modifier
//2.In rust-lang 'pub' is an access modifier of items in module { functions, structs, enums, traits, modules )
//3.In rust-lang only the public items of a module can be accessed from outside the module scope.
//4.In rust-lang module EBNF:
// <rust-module> ::= <access-modifier> <module-name> "{" <module-items>+ "}"
// <module-items> ::= { <module-item> }
// <module-item> ::=  <function> | <struct> | <enum> | <trait> | <module>

mod parent {

    fn info() {
        println!("function info() in module parent");
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

    mod nested {
        fn info() {
            println!("function info() in module nested, inside module parent");
        }
    }
}

fn main() {
    //call items of module parent
    // parent::info(); //compile error because of private visibility
    // let parent_pair= parent::Pair(0,1); //compile error because of private visibility
    // let parent_color= parent::Color::Red; //compile error because of private visibility

    // //implement trait for struct Pair
    // impl Greetable for parent::Pair {
    //     fn greet(&self) {
    //         println!("Implemented Greetable trait for struct Pair!");
    //     }
    // }
    // parent_pair.greet();  //compile error because of private visibility
    //
    // //call items of module parent
    // parent::nested::info(); //compile error because of private visibility

    println!("Module items need public visibility to be accessed from outside the module scope.");
}
