//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values

fn main() {
    let number = 3;

    //simple match expression
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //below match expression is used in assigment statement
    let val0 = match number {
        1 => {
            //do something here
            101 //If I place ';' match expression returns ().
        }

        2 => {
            //do something here
            102 //If I place ';' match expression returns ().
        }
        _ =>
        //do something here
        {
            111 //If I place ';' match expression returns ().
        }
    }; //Since is an expression I have to close it

    //below match expression is used in assigment statement
    let val1 = match number {
        1 => {
            //do something here
            //If I place ';' match expression returns ().
            101
        }
        2 => {
            //do something here
            //If I place ';' match expression returns ().
            102
        }
        3 => {
            //do something here
            //If I place ';' match expression returns ().
            103
        }
        _ => {
            //do something here
            //If I place ';' match expression returns ().
            111
        }
    }; //Since is an expression I have to close it

    println!("'match' return values: val0={}, val1 = {}", val0, val1);
}
