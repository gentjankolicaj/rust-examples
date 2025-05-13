//1.In rust-lang we use 'let else' construct to bind variables in scope else diverge(break,return,panic!)
//2.Binding of variables in scope can be of any type.

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

fn main() {
    let a = Color::GREEN;

    let let_else_binding = a else {
        panic!("error: failed to bind a to let_else_binding");
    };

    println!("The value of let_else_binding is: {:?}", let_else_binding);
}
