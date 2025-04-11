//1.In rust-lang we have variable 'shadowing'
//2.In rust-lang variable 'shadowing' is a case when on a block we have same identifier for different type variable (sort of redeclaration).
//3.The other case is when in block instead of having different type 'variable' we have reassignment.
//4.NOTE: IN RUST WE SHADOW BINDING BETWEEN IDENTIFIER AND VARIABLE
//5.So basically in RUST on variable bindings we have : ASSIGNMENT & SHADOWING.
//6.Example : we can shadow variable binding a with  [ let a; or let mut a;]

fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
