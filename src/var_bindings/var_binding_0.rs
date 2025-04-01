//1.Rust provides type safety via static typing.
//2.Variable bindings can be type 'annotated' when declared.
//3.Rust also can infer type of variable based on context.

fn main() {
    //variable type binding
    let a = 1i32; //int
    let b = 3.4f32; //float
    let c = (); //tuple

    //Compiler is able to infer type of variables base on context
    let d = a; //by compiler context inferring => d is i32
    let e = b; //by compiler context inferring => c is f32
    let f = c; //by compiler context inferring => f is ();

    println!("a:{},b:{},c:{:?}", a, b, c);
    println!("d:{},c:{},f:{:?}", d, e, f);
}
