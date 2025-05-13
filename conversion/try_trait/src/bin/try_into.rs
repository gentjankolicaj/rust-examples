//1.In rust we implement traits 'TryFrom' or 'TryInto' for fallible conversion.

use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber {
    value: i64,
}
impl TryInto<EvenNumber> for i32 {
    type Error = ();
    fn try_into(self) -> Result<EvenNumber, Self::Error> {
        if self % 2 == 0 {
            return Ok(EvenNumber { value: self as i64 });
        } else {
            return Err(());
        }
    }
}

impl TryInto<EvenNumber> for i64 {
    type Error = ();
    fn try_into(self) -> Result<EvenNumber, Self::Error> {
        if self % 2 == 0 {
            return Ok(EvenNumber { value: self });
        } else {
            return Err(());
        }
    }
}

fn main() {
    //primitive type variables
    let a = 1;
    let b = 2i64;

    //fallible variable with try_into()
    //It needs variable type after identifier else compiler throws exception.
    let res_a: Result<EvenNumber, ()> = a.try_into();
    let res_b: Result<EvenNumber, ()> = b.try_into();

    println!("primitives a={:?},b={:?}", a, b);
    println!(
        "fallible conversion result  res_a={:?},res_b={:?}",
        res_a, res_b
    );
}
