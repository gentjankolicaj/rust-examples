use std::fmt;

//1.In rust in order to allow conversion of type T to String we must implement 'ToString' trait or 'fmt::Display' trait which automatically provides ToString

//declare c_struct
struct Person {
    id: i64,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person(id={})", self.id)
    }
}

fn main() {
    //Instantiate Person
    let p0 = Person { id: 0 };
    let p1 = Person { id: 1 };

    println!("c-struct p0={},p1={}", p0, p1);
}
