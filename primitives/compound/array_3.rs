use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Pair(f32, f32);

//Define user
#[derive(Debug)]
struct User {
    id: i64,
    location: Point,
    age: u8,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User[id:{},point:@{},age:{}]",
            self.id, self.location, self.age
        )
    }
}

#[derive(Debug)]
struct Point {
    lat: f64,
    lon: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(lat:{},lon:{})", self.lat, self.lon)
    }
}

fn main() {
    let a = Pair(1.0, 2.0);
    println!("a {:?}", a);

    let b: [Pair; 4] = [Pair(3.0, 5.0); 4];
    println!("b {:?}", b);

    //create mutable variable
    let mut user = User {
        id: 0,
        location: Point { lat: 0.0, lon: 0.0 },
        age: 2,
    };
    println!("user: {}", user);

    //create a new struct and assign
    user = User {
        id: 1,
        location: Point { lat: 1.0, lon: 1.1 },
        age: 3,
    };
    println!("mutated user: {}", user);
}
