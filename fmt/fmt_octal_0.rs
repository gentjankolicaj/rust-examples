use std::fmt;

//1.Define a structure
//2.Derive fmt::Debug
//3.Implement fmt::Display for structure
//4.Implement fmt::Binary for structure
//5.Implement fmt::Octal for structure
//5.Print

#[derive(Debug)]
struct Person {
    id: i64,
    age: i8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fmt::Display Person  [id: {}, age: {}]",
            self.id, self.age
        )
    }
}

impl fmt::Binary for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fmt::Binary Person [{},{}]", self.id, self.age)
    }
}

impl fmt::Octal for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fmt::Octal Person [{},{}]", self.id, self.age)
    }
}
fn main() {
    let person = Person { id: 111, age: 30 };

    println!("Debug {:?}", person);
    println!("Display {}", person);
    println!("Binary {:b}", person);
    println!("Octal {:o}", person);
}
