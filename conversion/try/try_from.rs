//1.In rust we implement traits 'TryFrom' or 'TryInto' for fallible conversion.

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct EvenNumber(i64);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value as i64))
        } else {
            Err(())
        }
    }
}

impl TryFrom<i64> for EvenNumber {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(8i64), Ok(EvenNumber(8i64)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    assert_eq!(EvenNumber::try_from(5i64), Err(()));
}
