//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
//7.'match' can be used to destruct enums

//Define enum
#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}
fn main() {
    let car_color = Color::GREEN;

    println!("car_color value {:?}", car_color);

    match car_color {
        Color::GREEN => {
            println!("color green");
        }
        Color::RED => {
            println!("color red");
        }
        Color::BLUE => {
            println!("color blue");
        }
        _ => {
            //unreachable since Color enum has defined 4
            println!("unreachable");
        }
    }
}
