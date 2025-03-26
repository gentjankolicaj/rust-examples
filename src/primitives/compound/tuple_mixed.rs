//1.A tuple is a collection of values of different types.
//2.Tuples are constructed using parenthesis.
//3.Each tuple itself is a value with type signature (T1,T2,T3...) where T1,T2 are types of its members.
//4.Functions can use tuples to return values as tuples hold any number of values.

//Define struct
#[derive(Debug)]
struct User {
    id: i64,
    age: i8,
}
fn main() {
    let tuple_of_tuples = ((12, 3.145, 0u64, true), (true, false), 1u8);
    println!("tuple_of_tuples= {:?}", tuple_of_tuples);

    //mixed tuple
    let tuple_mixed = (
        (true, true, true),
        (101, User { id: 0, age: 10 }, true),
        10,
        3.24,
        User { id: 2, age: 20 },
    );
    println!("tuple_mixed= {:?}", tuple_mixed);
}
