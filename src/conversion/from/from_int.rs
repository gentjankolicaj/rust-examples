use std::convert::From;

//1.In rust, we implement trait 'From' in order to convert a custom type a to custom type B.
//2.Example : let b:B =B.from(a); where let a:a=value;

#[derive(Debug)]
struct Integer(i64);

impl From<i8> for Integer {
    fn from(param: i8) -> Self {
        return Integer(param as i64);
    }
}

impl From<i16> for Integer {
    fn from(param: i16) -> Self {
        return Integer(param as i64);
    }
}

impl From<i32> for Integer {
    fn from(param: i32) -> Self {
        return Integer(param as i64);
    }
}

impl From<i64> for Integer {
    fn from(param: i64) -> Self {
        return Integer(param);
    }
}

fn main() {
    let a = 1i8;
    let b = 2i16;
    let c = 3i32;
    let d = 4i64;

    //Integer instances from integer types
    let ia = Integer::from(a);
    let ib = Integer::from(b);
    let ic = Integer::from(c);
    let id = Integer::from(d);

    println!("a={},b={},c={},d={}", a, b, c, d);
    println!("ia={:?},ib={:?},ic={:?},id={:?}", ia, ib, ic, id);
}
