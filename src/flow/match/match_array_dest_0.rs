//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
fn main() {
    let array: [i32; 4] = [1, 2, 3, 4];

    println!("array values {:?}", array);

    //destruct array in match
    match array {
        //match only first index and any other
        [0, ..] => {
            println!("array[0]=0, and others={:?}", &array[1..4]);
        }
        //destruct only first index and any other
        [a, ..] => {
            println!("array[0]={}, and others={:?}", a, &array[1..4]);
        }
        //destruct first ^ second index and any other
        [a, b, ..] => {
            println!(
                "array[0]={},array[1]={}, and others={:?}",
                a,
                b,
                &array[1..4]
            );
        }
        //ignore first index and any other
        [_, ..] => {
            println!("ignore first index and any other");
        }
    }
}
