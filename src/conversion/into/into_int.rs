//1.In rust-lang we have interchangeable traits 'From' and 'Into' that we need to implement in order to convert custom types.
//2.For primitive types however conversion is done with casting.

#[derive(Debug)]
struct Integer(i64);

impl Into<Integer> for i8 {
    fn into(self) -> Integer {
        return Integer(self as i64);
    }
}

impl Into<Integer> for i16 {
    fn into(self) -> Integer {
        return Integer(self as i64);
    }
}

impl Into<Integer> for i32 {
    fn into(self) -> Integer {
        return Integer(self as i64);
    }
}
impl Into<Integer> for i64 {
    fn into(self) -> Integer {
        return Integer(self as i64);
    }
}

fn main() {
    //Primitive variable binding
    let a = 1i8;
    let b = 2i16;
    let c = 3i32;
    let d = 4i64;

    //Convert primitive type into custom type
    let ia: Integer = a.into();
    let ib: Integer = b.into();
    let ic: Integer = c.into();
    let id: Integer = d.into();

    println!("a={},b={},c={},d={}", a, b, c, d);
    println!("ia={:?},ib={:?},ic={:?},id={:?}", ia, ib, ic, id);
}
