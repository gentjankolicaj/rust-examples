//1.In rust-lang the use declaration is used to bring items—such as structs, enums, traits, functions, or modules—into scope, so you can refer to them with shorter paths.
//2.It is similar to import in other languages.

mod parent {

    pub mod nested {
        pub mod deep {
            pub fn a() {
                println!("direct-refer: deep nested function a(), parent::nested::deep::a()");
            }

            pub fn b() {
                println!("direct-refer: deep nested function b(), parent::nested::deep::b()");
            }

            pub fn c() {
                println!("direct-refer: deep nested function c(), parent::nested::deep::c()");
            }
        }
    }
}

fn main() {
    //call functions from deep nested module by directly referning to them
    parent::nested::deep::a();
    parent::nested::deep::b();
    parent::nested::deep::c();
}
