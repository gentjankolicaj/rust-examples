//1.In rust-lang we have interchangeable traits 'From' and 'Into' that we need to implement in order to convert custom types.
//2.For primitive types however conversion is done with casting.

use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        return Number { value: self };
    }
}

fn main() {
    //integer variable binding
    let a = 1;

    //Convert primitive type into custom type
    let b: Number = a.into();
    print!("primitive a={},custom-type b={:?}", a, b);
}
