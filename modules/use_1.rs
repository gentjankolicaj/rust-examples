//1.In rust-lang the use declaration is used to bring items—such as structs, enums, traits, functions, or modules—into scope, so you can refer to them with shorter paths.
//2.It is similar to import in other languages.

//import all functions from deep nested module
use parent::nested::deep::{a, b, c};

mod parent {

    pub mod nested {
        pub mod deep {
            pub fn a() {
                println!("multiple-import: deep nested function a(), parent::nested::deep::a()");
            }

            pub fn b() {
                println!("multiple-import: deep nested function b(), parent::nested::deep::b()");
            }

            pub fn c() {
                println!("multiple-import: deep nested function c(), parent::nested::deep::c()");
            }
        }
    }
}

fn main() {
    //call functions from deep nested module
    a();
    b();
    c();
}
