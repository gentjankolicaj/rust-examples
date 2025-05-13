//1.Structure is a custom type in rust
//2.There are 3 types of structures that can be created with 'struct' keyword.Tuple-struct,Unit-struct,C-struct.
//3.Tuple-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier(T1....Tn); Ex: struct Pair(f32,f32);
//4.Unit-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier; Ex: struct Unit;
//5.C-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier{ field1:T1, field2:T2,...}; Ex. struct User{ id: i64,age:u8};

//Below are defined different types of 'Tuple-struct'
#[derive(Debug)]
struct TuplePair(f64, f64);

#[derive(Debug)]
struct TuplePoint(f64, f64);

//Below are defined different types of 'Unit-struct'
#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Meter;

//Below are defined different 'C-struct' s.
#[derive(Debug)]
struct User {
    id: i64,
    name: String,
}

#[derive(Debug)]
struct UserLocation {
    id: i64,
    location: TuplePoint,
}

fn main() {
    //tuple-struct & print with debug
    let a = TuplePoint(10.0, 20.0);
    let b = TuplePair(10.0, 20.0);
    println!("a={:#?}", a);
    println!("b={:#?}", b);

    //unit-struct & print with debug
    let c = Unit;
    let d = Meter;
    println!("c={:#?}", c);
    println!("d={:#?}", d);

    //c-struct & print with debug
    let user = User {
        id: 0,
        name: "john-doe".to_string(),
    };
    let user_location = UserLocation {
        id: 0,
        location: TuplePoint(10.0, 20.0),
    };
    println!("user={:#?}", user);
    println!("user_location={:#?}", user_location);
}
