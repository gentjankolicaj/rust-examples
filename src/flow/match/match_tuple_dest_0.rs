//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples

fn main() {
    let pair = (10, 20);

    println!("tuple values {:?}", pair);

    //destruct tuple in match
    match pair {
        (0, b) => {
            println!(
                "tuple-struct vars: a={},b={} where b is destructed var of Pair",
                0, b
            );
        }
        (a, 0) => {
            println!(
                "tuple-struct vars: a={},b={} where a is destructed var of Pair",
                a, 0
            );
        }
        (a, b) => {
            //This is going to be printed because of full destructing tuple-struct
            println!(
                "tuple-struct vars: a={},b={} where a,b are destructed vars of Pair",
                a, b
            );
        }
        _ => {
            println!("unreachable because of (a,b) arm");
        }
    }
}
