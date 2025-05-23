//1.In rust-lang we have variable 'shadowing'
//2.In rust-lang variable 'shadowing' is a case when on a block we have same identifier for different type variable (sort of redeclaration).
//3.The other case is when in block instead of having different type 'variable' we have reassignment.
//4.NOTE: IN RUST WE SHADOW BINDING BETWEEN IDENTIFIER AND VARIABLE
//5.So basically in RUST on variable bindings we have : ASSIGNMENT & SHADOWING.
//6.Example : we can shadow variable binding a with  [ let a; or let mut a;]

fn main() {
    //declaration & initialization
    let variable_shadowing = 0;
    println!("variable_shadowing declared : {}", variable_shadowing);

    //first block
    {
        //variable shadowing here
        let variable_shadowing = "123r6";
        println!(
            "inner-block1  variable shadowing with &str {}",
            variable_shadowing
        );
    }

    //second block
    {
        //Second variable binding,shadowing mutable
        let mut variable_shadowing = 3.14;
        println!(
            "inner-block2 variable shadowing mutable with f32 {}",
            variable_shadowing
        );

        variable_shadowing = variable_shadowing + 1.0;
        println!(
            "inner-block2 variable shadowing mutable with f32 {}",
            variable_shadowing
        );
    }

    // This binding *shadows* the previous binding
    let variable_shadowing = 2;
    println!(
        "variable_shadowing shadowed in outer block: {}",
        variable_shadowing
    );
}
