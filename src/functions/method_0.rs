//1.In rust-lang functions are declared using the fn keyword.
//2.Some functions are connected to a particular type.
//3.These come in two forms: associated functions, and methods.
//4.Associated functions are functions that are defined on a type generally,
//5.Methods are associated functions that are called on a particular instance of a type.
//6.Methods have '&self' argument

struct Case {
    id: i32,
}

//implementation block where all Case methods
impl Case {
    fn case0(&self) {
        println!("id={},case0 no args,no returns", self.id);
    }

    fn case1(&self, arg0: bool, arg1: i8) {
        println!(
            "id={},case1 arg0={},arg1={}, no returns",
            self.id, arg0, arg1
        );
    }

    fn case2(&self, arg0: i8, arg1: i8) -> bool {
        println!(
            "id={},case2 arg0={},arg1={}, bool return",
            self.id, arg0, arg1
        );

        //final expression in fn is used as return value
        return true;
    }
}

fn main() {
    //instantiate Case
    let case_instance = Case { id: 111 };

    //call methods
    case_instance.case0();
    case_instance.case1(true, 8);
    case_instance.case2(0, 1);
}
