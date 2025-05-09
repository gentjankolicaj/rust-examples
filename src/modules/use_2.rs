//1.In rust-lang the use declaration is used to bring items—such as structs, enums, traits, functions, or modules—into scope, so you can refer to them with shorter paths.
//2.It is similar to import in other languages.

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}
