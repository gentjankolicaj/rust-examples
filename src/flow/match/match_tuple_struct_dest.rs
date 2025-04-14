//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct 'tuple-struct'

//Define tuple struct
struct Pair(i32, i32);
fn main() {
    let pair = Pair(0, 101);

    //destruct tuple in match
    match pair {
        Pair(0, b) => {
            //This is going to be printed because of a destructing
            println!(
                "tuple vars: a={},b={} where b is destructed var of tuple",
                0, b
            );
        }
        Pair(a, 0) => {
            println!(
                "tuple vars: a={},b={} where a is destructed var of tuple",
                a, 0
            );
        }
        Pair(a, b) => {
            println!(
                "tuple vars: a={},b={} where a,b are destructed vars of tuple",
                a, b
            );
        }
        Pair(_, _) => {
            println!("unreachable because of Pair(a,b) arm");
        }
    }
}
