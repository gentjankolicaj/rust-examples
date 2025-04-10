use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ToString;

//Declare c-struct
struct Person(i32);
impl FromStr for Person {
    //declare a alias for ParseIntError
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<i32>() {
            Ok(id) => Ok(Person(id)),
            Err(e) => Err(e),
        }
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("id:{}", self.0);
    }
}

fn main() {
    //declare string literal
    let lit = "   333  ";
    let person: Person = lit.parse().unwrap();

    println!("string literal {}", lit);
    println!("Person instance to_string() {}", person.to_string());
}
