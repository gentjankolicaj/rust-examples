//1.Structure is a custom type in rust
//2.There are 3 types of structures that can be created with 'struct' keyword.Tuple-struct,Unit-struct,C-struct.
//3.Tuple-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier(T1....Tn); Ex: struct Pair(f32,f32);
//4.Unit-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier; Ex: struct Unit;
//5.C-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier{ field1:T1, field2:T2,...}; Ex. struct User{ id: i64,age:u8};

//Below are different types of 'unit-struct'
#[derive(Debug, Copy, Clone)]
struct Unit;

#[derive(Debug, Copy, Clone)]
struct Meter;

#[derive(Debug, Copy, Clone)]
struct Kilometer;

fn main() {
    let unit = Unit;
    let meter = Meter;
    let kilo = Kilometer;

    println!("{:?}", unit);
    println!("{:?}", meter);
    println!("{:?}", kilo);
}
