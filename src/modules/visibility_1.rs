//1.In rust-lang by default, the items in a module have private visibility, but this can be overridden with the pub modifier
//2.In rust-lang 'pub' is an access modifier of items in module { functions, structs, enums, traits, modules )
//3.In rust-lang only the public items of a module can be accessed from outside the module scope.
//4.In rust-lang module EBNF:
// <rust-module> ::= <access-modifier> <module-name> "{" <module-items>+ "}"
// <module-items> ::= { <module-item> }
// <module-item> ::=  <function> | <struct> | <enum> | <trait> | <module>

use parent::Greetable;

mod parent {

    pub fn info() {
        println!("function info() in module parent");
    }

    // Items in modules default to private visibility.
    fn private_func() {
        println!("called `parent::private_func()`");
    }

    pub fn info_indirect() {
        println!("called `parent::info_private()`");
        private_func();

        nested::info2();
    }

    //tuple-like struct
    #[derive(Debug)]
    pub struct Pair(pub i32, pub i32);

    #[derive(Debug)]
    pub enum Color {
        Red = 0,
        Green = 1,
        Blue = 2,
    }

    pub trait Greetable {
        fn greet(&self);
    }

    pub mod nested {
        pub fn info() {
            println!("function info() in module nested, inside module parent");
        }
        // Functions declared using `pub(super)` syntax are only visible within,the parent module
        pub(super) fn info2() {
            println!("function info2() in module 'nested', inside module 'parent'.pub(super)=>only visible within,the 'parent' module");
        }
    }
}

fn main() {
    //call items of module parent
    parent::info();
    parent::info_indirect();
    let parent_pair = parent::Pair(0, 1);
    let parent_color = parent::Color::Red;
    println!("parent_color = {:?}", parent_color);
    println!("parent_pair = {:?}", parent_pair);

    //implement trait for struct Pair
    impl Greetable for parent::Pair {
        fn greet(&self) {
            println!("Implemented Greetable trait for struct Pair!");
        }
    }
    parent_pair.greet();

    //call items of module nested
    parent::nested::info();
}
