//1.In rust-lang functions are declared using the fn keyword.
//2.Some functions are connected to a particular type.
//3.These come in two forms: associated functions, and methods.
//4.Associated functions are functions that are defined on a type generally,
//5.Methods are associated functions that are called on a particular instance of a type.

struct Case {
    id: i32,
}

//implementation block where all Case associated functions
impl Case {
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
}

fn main() {
    //instantiate Case
    let case_instance = Case { id: 0 };

    //call associated functions
    Case::case0();
    Case::case1(true, 8);
    Case::case2(0, 1);
}
