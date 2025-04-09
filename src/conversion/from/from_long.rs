use std::convert::From;

//1.In rust, we implement trait 'From' in order to convert a custom type A to custom type B.
//2.Example : let b:B =B.from(a); where let a:A=value;

#[derive(Debug)]
struct Long {
    value: i128,
}

impl From<i8> for Long {
    fn from(param: i8) -> Self {
        return Long {
            value: param as i128,
        };
    }
}

impl From<i16> for Long {
    fn from(param: i16) -> Self {
        return Long {
            value: param as i128,
        };
    }
}

impl From<i32> for Long {
    fn from(param: i32) -> Self {
        return Long {
            value: param as i128,
        };
    }
}

impl From<i64> for Long {
    fn from(param: i64) -> Self {
        return Long {
            value: param as i128,
        };
    }
}

impl From<i128> for Long {
    fn from(param: i128) -> Self {
        return Long {
            value: param as i128,
        };
    }
}

fn main() {
    let a = 1i8;
    let b = 2i16;
    let c = 3i32;
    let d = 4i64;
    let e = 5i128;

    //Long instances from integer types.
    let la = Long::from(a);
    let lb = Long::from(b);
    let lc = Long::from(c);
    let ld = Long::from(d);
    let le = Long::from(e);

    println!("a={},b={},c={},d={},e={}", a, b, c, d, e);
    println!(
        "la={:?},lb={:?},lc={:?},ld={:?},le={:?}",
        la, lb, lc, ld, le
    );
}
