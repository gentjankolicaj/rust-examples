use std::convert::From;

//1.In rust, we implement trait 'From' in order to convert a custom type A to custom type B.
//2.Example : let b:B =B.from(a); where let a:A=value;

#[derive(Debug)]
struct Integer(i64);

impl From<i64> for Integer {
    fn from(param: i64) -> Self {
        return Integer(param);
    }
}

fn main() {
    let a: i64 = 101;
    let ia = Integer::from(a);
    println!("a={:?},ia={:?}", a, ia);
}
