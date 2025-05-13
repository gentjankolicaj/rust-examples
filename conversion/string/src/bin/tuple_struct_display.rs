use std::fmt;

//1.In rust in order to allow conversion of type T to String we must implement 'ToString' trait or 'fmt::Display' trait which automatically provides ToString

//Declare tuple-struct
struct Person(i64);

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person(id={})", self.0)
    }
}

fn main() {
    //Instantiate Person
    let p0 = Person(0);
    let p1 = Person(1);

    println!("tuple-struct p0={},p1={}", p0, p1);
}
