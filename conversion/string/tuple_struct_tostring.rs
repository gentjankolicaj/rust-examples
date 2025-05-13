//1.In rust in order to allow conversion of type T to String we must implement 'ToString' trait or 'fmt::Display' trait which automatically provides ToString

//Declare tuple-struct
struct Person(i64);

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("id:{}", self.0);
    }
}

fn main() {
    //Instantiate Person
    let p0 = Person(0);
    let p1 = Person(1);

    println!("tuple-struct p0={},p1={}", p0.to_string(), p1.to_string());
}
