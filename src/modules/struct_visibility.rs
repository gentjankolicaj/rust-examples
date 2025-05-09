//1.In rust-lang by default, the items in a module have private visibility, but this can be overridden with the pub modifier
//2.In rust-lang 'pub' is an access modifier of items in module { functions, structs, enums, traits, modules )
//3.In rust-lang only the public items of a module can be accessed from outside the module scope.
//4.In rust-lang module EBNF:
// <rust-module> ::= <access-modifier> <module-name> "{" <module-items>+ "}"
// <module-items> ::= { <module-item> }
// <module-item> ::=  <function> | <struct> | <enum> | <trait> | <module>
//5.In rust-lang structs have an extra level of visibility with their fields
//6.In rust-lang struct field visibility defaults to private, and can be overridden with the pub modifier


mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}