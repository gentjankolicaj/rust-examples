//1.In rust-lang 'while' keyword is used to run a loop while a condition is true
//2.'while' statement BNF =>
// <while-statement> ::= "while" <while-expression> <while-body>
// <while-expression> ::= <rust-expression> return ( true | false )
// <while-body> ::= *<rust-statement>
// <rust-statement> ::= *<rust-statement> | *<rust-expression>

fn eval(param: i32) -> bool {
    if param < 100 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut counter = 0;

    while eval(counter) {
        counter = counter + 1;
        println!("counter is {}", counter);

        if counter > 100 {
            println!("About to break while loop");
            break;
        }
    }
}
