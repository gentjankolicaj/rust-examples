//1.Structure is a custom type in rust
//2.There are 3 types of structures that can be created with 'struct' keyword.Tuple-struct,Unit-struct,C-struct.
//3.Tuple-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier(T1....Tn); Ex: struct Pair(f32,f32);
//4.Unit-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier; Ex: struct Unit;
//5.C-struct is a type of struct that can be created with struct keyword.BNF => struct Identifier{ field1:T1, field2:T2,...}; Ex. struct User{ id: i64,age:u8};

//Below are different types of 'tuple-struct'
#[derive(Debug, Copy, Clone)]
struct Unary(i32);

#[derive(Debug, Copy, Clone)]
struct Pair(usize, i32);

#[derive(Debug, Copy, Clone)]
struct Map<'a>(&'a [Pair]); // struct of type tuple with a slice of pair types

fn main() {
    let unary = Unary(1);
    println!("unary={:#?}", unary);

    let pair = Pair(1, 1);
    let mut pair_array: [Pair; 10] = [pair; 10];
    println!("pair_array={:#?}", pair_array);
    //update pair_array
    for i in 5..pair_array.len() {
        pair_array[i] = Pair(i, 0);
    }
    println!("Updated pair_array={:#?}", pair_array);

    let left_map = Map(&pair_array[0..5]);
    let right_map = Map(&pair_array[5..10]);
    println!("left_map={:#?}", left_map);
    println!("right_map={:#?}", right_map);
}
