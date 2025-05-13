//1.Structure is a custom type in rust
//2.There are 3 types of structures that can be created with 'struct' keyword.Tuple-struct,Unit-struct,C-struct.
//3.Tuple-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier(T1....Tn); Ex: struct Pair(f32,f32);
//4.Unit-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier; Ex: struct Unit;
//5.C-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier{ field1:T1, field2:T2,...}; Ex. struct User{ id: i64,age:u8};

//Below are different types of 'c-struct'

#[derive(Debug, Copy, Clone)]
struct Unit;

#[derive(Debug, Copy, Clone)]
struct Meter;

#[derive(Debug, Copy, Clone)]
struct Road(i64, Meter);

#[derive(Debug, Copy, Clone)]
struct House<'a> {
    id: i64,
    name: &'a str,
    road: Road,
}

fn main() {
    let meter = Meter;
    let house_road = Road(101, meter);
    let house = House {
        id: 0,
        name: "House",
        road: house_road,
    };

    println!("{:#?}", house);
}
