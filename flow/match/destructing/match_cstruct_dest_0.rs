//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
//7.'match' can be used to destruct enums
//8.'match' can be used to destruct c-structs

//Define c-struct
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let center = Coord { x: 0, y: 0 };

    //destruct c-struct with match
    match center {
        Coord { x: 0, y: y } => {
            println!("Coord.y destructed , x={}, y={}", 0, y);
        }
        Coord { x: x, y: 0 } => {
            println!("Coord.x destructed , x={}, y={}", x, 0);
        }
        Coord { x: x, y: y } => {
            //this is going to get printed
            println!("Coord x,y destructed x = {}, y = {}", x, y)
        }
        Coord { x: _, y: _ } => {
            println!("Coord _,_ unreachable");
        }
    }

    let one = Coord { x: 1, y: 1 };

    //destruct c-struct with match
    match one {
        Coord { x: 0, y: y } => {
            println!("Coord.y destructed , x={}, y={}", 0, y);
        }
        Coord { x: x, y: 0 } => {
            println!("Coord.x destructed , x={}, y={}", x, 0);
        }
        Coord { x: x, y: y } => {
            //this is going to get printed
            println!("Coord x,y destructed x = {}, y = {}", x, y)
        }
        Coord { x: _, y: _ } => {
            println!("Coord _,_ unreachable");
        }
    }
}
