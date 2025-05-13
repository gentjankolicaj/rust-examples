//1.In rust-lang functions are declared using the fn keyword.
//2.Some functions are connected to a particular type.
//3.These come in two forms: associated functions, and methods.
//4.Associated functions are functions that are defined on a type generally,
//5.Methods are associated functions that are called on a particular instance of a type.
//6.Methods have '&self' argument

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

//implementation block where all Color methods
impl Color {
    fn case0(&self) {
        println!("color={:?},case0 no args,no returns", self);
    }

    fn case1(&self, arg0: bool, arg1: i8) {
        println!(
            "color={:?},case1 arg0={},arg1={}, no returns",
            self, arg0, arg1
        );
    }

    fn case2(&self, arg0: i8, arg1: i8) -> bool {
        println!(
            "color={:?},case2 arg0={},arg1={}, bool return",
            self, arg0, arg1
        );

        //final expression in fn is used as return value
        return true;
    }
}

fn main() {
    //non mutable variable red of type Color (enum)
    let red = Color::RED;

    //call methods
    red.case0();
    red.case1(true, 8);
    red.case2(0, 1);
}
