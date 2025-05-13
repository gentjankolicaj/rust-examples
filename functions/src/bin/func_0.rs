//1.In rust-lang functions are declared using the fn keyword.
//2.Rust functions arguments are type annotated, just like variables, and, if the function returns a value, the return type must be specified after an arrow ->.
//3.Rust functions the final expression in the function will be used as return value.
//4.Rust alternatively, the return statement can be used to return a value earlier from within the function, even from inside loops or if statements.

fn case0() {
    println!("case0 no args,no returns");
}

fn case1(arg0: bool, arg1: i8) {
    println!("case1 arg0={},arg1={}, no returns", arg0, arg1);
}

fn case2(arg0: i8, arg1: i8) -> bool {
    println!("case2 arg0={},arg1={}, bool return", arg0, arg1);

    //final expression in fn is used as return value
    return true;
}

fn main() {
    //call declared functions
    case0();
    case1(true, 8);
    case2(0, 1);
}
