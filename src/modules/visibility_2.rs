//1.In rust-lang by default, the items in a module have private visibility, but this can be overridden with the pub modifier
//2.In rust-lang 'pub' is an access modifier of items in module { functions, structs, enums, traits, modules )
//3.In rust-lang only the public items of a module can be accessed from outside the module scope.
//4.In rust-lang module EBNF:
// <rust-module> ::= <access-modifier> <module-name> "{" <module-items>+ "}"
// <module-items> ::= { <module-item> }
// <module-item> ::=  <function> | <struct> | <enum> | <trait> | <module>
//5.In rust-lang structs have an extra level of visibility with their fields
//6.In rust-lang struct field visibility defaults to private, and can be overridden with the pub modifier

mod models {

    //unit-like struct
    #[derive(Debug)]
    pub struct Unit;

    //tuple-like struct
    #[derive(Debug)]
    pub struct Pair(pub i32, pub i32);

    //c-like struct
    #[derive(Debug)]
    pub struct User {
        pub name: String,
        pub age: u8,
    }

    //tuple-like struct
    //can't access fields since they are private
    #[allow(dead_code)]
    pub struct PairPrivateFields(i32, i32);

    //c-like struct
    //can't access fields since they are private
    #[allow(dead_code)]
    pub struct UserPrivateFields {
        name: String,
        age: u8,
    }
}

fn main() {
    //call members of a module 'models'
    let unit = models::Unit;
    let pair = models::Pair(1, 2);
    let user = models::User {
        name: "John".to_string(),
        age: 20,
    };

    //let pair_private_fields=models::PairPrivateFields(1,2); //compile error because of fields are private
    // let user_private_fields=models::UserPrivateFields{name:"John".to_string(),age:20}; //compile error because of fields are private

    println!("{:?}", unit);
    println!("{:?}", pair);
    println!("{:?}", user);
}
