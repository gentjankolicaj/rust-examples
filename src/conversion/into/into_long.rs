//1.In Rust we have interchangeable traits 'From' and 'Into' that we need to implement in order to convert custom types.
//2.For primitive types however conversion is done with casting.

#[derive(Debug)]
struct Long {
    value: i128,
}

impl Into<Long> for i8 {
    fn into(self) -> Long {
        return Long {
            value: self as i128,
        };
    }
}

impl Into<Long> for i16 {
    fn into(self) -> Long {
        return Long {
            value: self as i128,
        };
    }
}

impl Into<Long> for i32 {
    fn into(self) -> Long {
        return Long {
            value: self as i128,
        };
    }
}
impl Into<Long> for i64 {
    fn into(self) -> Long {
        return Long {
            value: self as i128,
        };
    }
}

fn main() {
    //Primitive variable binding
    let a = 1i8;
    let b = 2i16;
    let c = 3i32;
    let d = 4i64;

    //Convert primitive type into custom type
    let la: Long = a.into();
    let lb: Long = b.into();
    let lc: Long = c.into();
    let ld: Long = d.into();

    println!("a={},b={},c={},d={}", a, b, c, d);
    println!("la={:?},lb={:?},lc={:?},ld={:?}", la, lb, lc, ld);
}
